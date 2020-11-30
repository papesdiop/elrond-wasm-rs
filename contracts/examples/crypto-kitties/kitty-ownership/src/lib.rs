#![no_std]

imports!();

use kitty::{Kitty, KittyGenesType};

#[elrond_wasm_derive::contract(KittyOwnershipImpl)]
pub trait KittyOwnership {
	#[init]
	fn init(&self, gene_science_contract_address: &Address, auto_breed_fee: BigUint,
			_gen_zero_kitties: u32) {

		self.set_gene_science_contract_address(gene_science_contract_address);
		self.set_auto_birth_fee(auto_breed_fee);
		self._create_genesis_kitty();

		// TBD: randomly create gen_zero_kitties
	}

	// endpoints - ERC721 required

	#[endpoint(totalSupply)]
	fn total_supply(&self) -> u32 {
		self.get_total_kitties() - 1 // not counting genesis Kitty
	}

	#[endpoint(balanceOf)]
	fn balance_of(&self, address: &Address) -> u32 {
		self.get_nr_owned_kitties(address)
	}

	#[endpoint(ownerOf)]
	fn owner_of(&self, kitty_id: u32) -> Address {
		self.get_kitty_owner(kitty_id)
	}

	#[endpoint]
	fn approve(&self, to: &Address, kitty_id: u32) -> SCResult<()> {
		let caller = self.get_caller();

		require!(self.get_kitty_owner(kitty_id) == caller,
			"You are not the owner of that kitty!");

		self.set_approved_address(kitty_id, to);
		self.approve_event(&caller, to, kitty_id);

		Ok(())
	}

	#[endpoint]
	fn transfer(&self, to: &Address, kitty_id: u32) -> SCResult<()> {
		let caller = self.get_caller();

		require!(to != &Address::zero(), "Can't transfer to default address 0x0!");
		require!(to != &self.get_sc_address(), "Can't transfer to this contract!");

		require!(self.get_kitty_owner(kitty_id) == caller, 
			"You are not the owner of that kitty!");

		self._transfer(&caller, to, kitty_id);

		Ok(())
	}

	#[endpoint]
	fn transfer_from(&self, from: &Address, to: &Address, kitty_id: u32) -> SCResult<()> {
		let caller = self.get_caller();

		require!(to != &Address::zero(), "Can't transfer to default address 0x0!");
		require!(to != &self.get_sc_address(), "Can't transfer to this contract!");

		require!(&self.get_kitty_owner(kitty_id) == from,
			"Address _from_ is not the owner!");

		require!(self.get_kitty_owner(kitty_id) == caller ||
			self.get_approved_address(kitty_id) == caller,
			"You are not the owner of that kitty nor the approved address!");

		self._transfer(from, to, kitty_id);

		Ok(())
	}

	#[endpoint(tokensOfOwner)]
	fn tokens_of_owner(&self, address: &Address) -> Vec<u32> {
		let nr_owned_kitties = self.get_nr_owned_kitties(address);
		let total_kitties = self.get_total_kitties();
		let mut kitty_list = Vec::new();

		for kitty_id in 1..total_kitties {
			if nr_owned_kitties as usize == kitty_list.len() {
				break;
			}

			if &self.get_kitty_owner(kitty_id) == address {
				kitty_list.push(kitty_id);
			}
		}
		
		kitty_list
	}

	// endpoints - Kitty Breeding

	#[endpoint(approveSiring)]
	fn approve_siring(&self, kitty_id: u32, address: &Address) -> SCResult<()> {
		require!(self.get_kitty_owner(kitty_id) == self.get_caller(), 
			"You are not the owner of the kitty!");

		self.set_sire_allowed_address(kitty_id, address);

		Ok(())
	}

	#[endpoint(isReadyToBreed)]
	fn is_ready_to_breed(&self, kitty_id: u32) -> SCResult<bool> {
		require!(kitty_id != 0, "Invalid kitty id");

		let kitty = self.get_kitty_by_id(kitty_id);

		Ok(self._is_ready_to_breed(&kitty))
	}

	#[endpoint(isPregnant)]
	fn is_pregnant(&self, kitty_id: u32) -> SCResult<bool> {
		require!(kitty_id != 0, "Invalid kitty id");

		let kitty = self.get_kitty_by_id(kitty_id);

		Ok(kitty.siring_with_id != 0)
	}

	#[endpoint(canBreedWith)]
	fn can_breed_with(&self, matron_id: u32, sire_id: u32) -> SCResult<bool> {
		require!(matron_id != 0, "Invalid matron id!");
		require!(sire_id != 0, "Invalid sire id!");

		Ok(self._is_valid_mating_pair(matron_id, sire_id) && 
			self._is_siring_permitted(matron_id, sire_id))
	}

	#[payable]
	#[endpoint(breedWithAuto)]
	fn breed_with_auto(&self, #[payment] payment: BigUint, matron_id: u32, sire_id: u32) 
		-> SCResult<()> {

		let auto_birth_fee = self.get_auto_birth_fee();
		let caller = self.get_caller();

		require!(payment == auto_birth_fee, "Wrong fee!");
	
		require!(caller == self.get_kitty_owner(matron_id), 
			"You are not the owner of the kitty!");

		require!(self._is_siring_permitted(matron_id, sire_id), 
			"Siring not permitted!");

		let matron = self.get_kitty_by_id(matron_id);
		let sire = self.get_kitty_by_id(sire_id);

		require!(self._is_ready_to_breed(&matron), 
			"Matron not ready to breed!");

		require!(self._is_ready_to_breed(&sire), 
			"Sire not ready to breed!");

		require!(self._is_valid_mating_pair(matron_id, sire_id), 
			"Not a valid mating pair!");

		self._breed(matron_id, sire_id);

		Ok(())
	}

	#[endpoint(giveBirth)]
	fn give_birth(&self, matron_id: u32) -> SCResult<u32> {
		let total_kitties = self.get_total_kitties();

		require!(matron_id < total_kitties, "Invalid matron ID!");

		let mut matron = self.get_kitty_by_id(matron_id);

		require!(self._is_ready_to_give_birth(&matron), 
			"Matron not ready to give birth!");

		let sire_id = matron.siring_with_id;
		let sire = self.get_kitty_by_id(sire_id);
		let new_kitty_generation: u16;

		if matron.generation > sire.generation {
			new_kitty_generation = matron.generation + 1;
		}
		else {
			new_kitty_generation = sire.generation + 1;
		}

		// TBD
		let new_kitty_genes: KittyGenesType = 0;

		// new kitty goes to the owner of the matron
		let new_kitty_owner = self.get_kitty_owner(matron_id);
		let new_kitty_id = self._create_new_kitty(matron_id, sire_id, 
			new_kitty_generation, new_kitty_genes, &new_kitty_owner);

		// update matron kitty
		matron.siring_with_id = 0;
		self.set_kitty_at_id(matron_id, &matron);

		// send auto birth fee to caller
		let caller = self.get_caller();
		let fee = self.get_auto_birth_fee();
		self.send_tx(&caller, &fee, "Auto Birth Fee");

		Ok(new_kitty_id)
	}

	// private

	fn _transfer(&self, from: &Address, to: &Address, kitty_id: u32) {
		let mut nr_owned_from = self.get_nr_owned_kitties(from);
		nr_owned_from -= 1;

		if from != &Address::zero() {
			let mut nr_owned_to = self.get_nr_owned_kitties(to);
			nr_owned_to += 1;

			self.set_nr_owned_kitties(to, nr_owned_to);
			self.clear_sire_allowed_address(kitty_id);
			self.clear_approved_address(kitty_id);
		}
		
		self.set_nr_owned_kitties(from, nr_owned_from);
		self.set_kitty_owner(kitty_id, to);

		self.transfer_event(from, to, kitty_id);
	}

	// checks should be done in the caller function
	// returns the newly created kitten id
	fn _create_new_kitty(&self, matron_id: u32, sire_id: u32, generation: u16, 
		genes: KittyGenesType, owner: &Address) -> u32 {

		let mut total_kitties = self.get_total_kitties();
		let new_kitty_id = total_kitties;
		let kitty = Kitty {
			genes,
			birth_time: self.get_block_timestamp(),
			cooldown_end: 0,
			matron_id,
			sire_id,
			siring_with_id: 0,
			nr_children: 0,
			generation
		};

		total_kitties += 1;
		self.set_total_kitties(total_kitties);
		self.set_kitty_at_id(new_kitty_id, &kitty);
		
		self._transfer(&Address::zero(), owner, new_kitty_id);

		new_kitty_id
	}

	fn _create_genesis_kitty(&self) {
		let genesis_kitty = Kitty::default();

		self._create_new_kitty(genesis_kitty.matron_id, genesis_kitty.sire_id, 
			genesis_kitty.generation, genesis_kitty.genes, &self.get_sc_address());
	}

	fn _trigger_cooldown(&self, kitty: &mut Kitty) {
		let cooldown = kitty.get_next_cooldown_time();
		kitty.cooldown_end = self.get_block_timestamp() + cooldown;
	}

	fn _breed(&self, matron_id: u32, sire_id: u32) {
		let mut matron = self.get_kitty_by_id(matron_id);
		let mut sire = self.get_kitty_by_id(sire_id);

		// mark matron as pregnant
		matron.siring_with_id = sire_id;

		self._trigger_cooldown(&mut matron);
		self._trigger_cooldown(&mut sire);

		self.clear_sire_allowed_address(matron_id);
		self.clear_sire_allowed_address(sire_id);

		self.set_kitty_at_id(matron_id, &matron);
		self.set_kitty_at_id(sire_id, &sire);
	}

	// private - Kitty checks. These should be in the Kitty struct,
	// but unfortunately, they need access to the contract-only functions

	fn _is_ready_to_breed(&self, kitty: &Kitty) -> bool {
		kitty.siring_with_id == 0 && kitty.cooldown_end <= self.get_block_timestamp()
	}

	fn _is_siring_permitted(&self, matron_id: u32, sire_id: u32) -> bool {
		let sire_owner = self.get_kitty_owner(sire_id);
		let matron_owner = self.get_kitty_owner(matron_id);
		let sire_approved_address = self.get_sire_allowed_address(sire_id);

		sire_owner == matron_owner || matron_owner == sire_approved_address
	}

	fn _is_ready_to_give_birth(&self, matron: &Kitty) -> bool {
		matron.siring_with_id != 0 && matron.cooldown_end <= self.get_block_timestamp()
	}

	fn _is_valid_mating_pair(&self, matron_id: u32, sire_id: u32) -> bool {
		let matron = self.get_kitty_by_id(matron_id);
		let sire = self.get_kitty_by_id(sire_id);

		// can't breed with itself
		if matron_id == sire_id {
			return false;
		}

		// can't breed with their parents
		if matron.matron_id == sire_id || matron.sire_id == sire_id {
			return false;
		}
		if sire.matron_id == matron_id || sire.sire_id == matron_id {
			return false;
		}

		// for gen zero kitties
		if sire.matron_id == 0 || matron.matron_id == 0 {
			return true;
		}

		// can't breed with full or half siblings
		if sire.matron_id == matron.matron_id || sire.matron_id == matron.sire_id {
			return false;
		}
		if sire.sire_id == matron.matron_id || sire.sire_id == matron.sire_id {
			return false;
		}

		return true;
	}

	// storage - General

	#[storage_get("geneScienceContractAddress")]
	fn get_gene_science_contract_address(&self) -> Address;

	#[storage_set("geneScienceContractAddress")]
	fn set_gene_science_contract_address(&self, address: &Address);

	#[storage_get("autoBirthFee")]
	fn get_auto_birth_fee(&self) -> BigUint;

	#[storage_set("autoBirthFee")]
	fn set_auto_birth_fee(&self, fee: BigUint);

	// storage - Kitties

	#[storage_get("totalKitties")]
	fn get_total_kitties(&self) ->u32;

	#[storage_set("totalKitties")]
	fn set_total_kitties(&self, total_kitties: u32);

	#[storage_get("kitty")]
	fn get_kitty_by_id(&self, kitty_id: u32) -> Kitty;

	#[storage_set("kitty")]
	fn set_kitty_at_id(&self, kitty_id: u32, kitty: &Kitty);

	#[view(getKittyOwner)]
	#[storage_get("owner")]
	fn get_kitty_owner(&self, kitty_id: u32) -> Address;

	#[storage_set("owner")]
	fn set_kitty_owner(&self, kitty_id: u32, owner: &Address);

	#[storage_get("nrOwnedKitties")]
	fn get_nr_owned_kitties(&self, address: &Address) -> u32;

	#[storage_set("nrOwnedKitties")]
	fn set_nr_owned_kitties(&self, address: &Address, nr_owned: u32);

	#[view(getApprovedAddressForKitty)]
	#[storage_get("approvedAddress")]
	fn get_approved_address(&self, kitty_id: u32) -> Address;

	#[storage_set("approvedAddress")]
	fn set_approved_address(&self, kitty_id: u32, address: &Address);

	#[storage_clear("approvedAddress")]
	fn clear_approved_address(&self, kitty_id: u32);

	#[view(getSireAllowedAddressForKitty)]
	#[storage_get("sireAllowedAddress")]
	fn get_sire_allowed_address(&self, kitty_id: u32) -> Address;

	#[storage_set("sireAllowedAddress")]
	fn set_sire_allowed_address(&self, kitty_id: u32, address: &Address);

	#[storage_clear("sireAllowedAddress")]
	fn clear_sire_allowed_address(&self, kitty_id: u32);

	// events

	#[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
	fn transfer_event(&self, from: &Address, to: &Address, token_id: u32);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000002")]
	fn approve_event(&self, owner: &Address, approved: &Address, token_id: u32);
}
