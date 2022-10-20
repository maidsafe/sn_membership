use crate::mvba::{
    abba::ABBA, broadcaster::Broadcaster, crypto::public::PubKey, proposal::Proposal, vcbc::VCBC,
    ProposalChecker,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Consensus {
    id: u32,
    self_key: PubKey,
    abba: ABBA,
    threshold: usize,
    vcbc_map: HashMap<PubKey, VCBC>,
    broadcaster: Rc<RefCell<Broadcaster>>,
}

impl Consensus {
    pub fn init(
        id: u32,
        self_key: PubKey,
        parties: Vec<PubKey>,
        threshold: usize,
        proposal_checker: ProposalChecker,
    ) -> Consensus {
        let abba = ABBA::new(); // TODO: Vec<> ???
        let mut vcbc_map = HashMap::new();
        let broadcaster = Broadcaster::new(id, &self_key);
        let broadcaster_rc = Rc::new(RefCell::new(broadcaster));
        let proposal_checker_rc = Rc::new(RefCell::new(proposal_checker));

        for p in &parties {
            let vcbc = VCBC::new(
                &p,
                &parties,
                threshold,
                broadcaster_rc.clone(),
                proposal_checker_rc.clone(),

            );
            vcbc_map.insert(p.clone(), vcbc).unwrap();
        }

        Consensus {
            id,
            self_key,
            abba,
            threshold,
            vcbc_map,
            broadcaster: broadcaster_rc,
        }
    }

    // start the consensus by proposing a proposal and broadcasting it.
    pub fn start(&mut self, proposal: Proposal) -> Vec<Vec<u8>> {
        let vcbc = self.vcbc_map.get_mut(&self.self_key).unwrap(); // TODO: no unwrap
        vcbc.propose(&proposal).unwrap(); // TODO: no unwrap

        self.broadcaster.borrow_mut().take_bundles()
    }

    pub fn process_bundle(&mut self, data: &[u8]) -> Vec<Vec<u8>> {
        
        let mut delivered_count = 0;
        for (_, vcbc) in &self.vcbc_map {
            if vcbc.is_delivered() {
                delivered_count+=1;
            }
        }

        if delivered_count>=self.super_majority_num() {

        }


        self.broadcaster.borrow_mut().take_bundles()
    }

    fn super_majority_num(&self) -> usize {
        self.vcbc_map.len() - self.threshold
    }
}
