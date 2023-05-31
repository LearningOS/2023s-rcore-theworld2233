use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

/// Deadlock checker
#[derive(Debug)]
pub struct DeadlockChecker {
    /// available vector
    pub available: Vec<usize>,
    /// allocation matrix
    pub allocation: BTreeMap<usize, BTreeMap<usize, usize>>,
    /// need matrix
    pub need: BTreeMap<usize, BTreeMap<usize, usize>>,
}

impl DeadlockChecker {
    /// Init a deadlock checker
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            allocation: BTreeMap::new(),
            need: BTreeMap::new(),
        }
    }

    /// Increase available vector
    pub fn inc_available(&mut self, res_id: usize, cnt: usize) {
        self.available[res_id] += cnt;
    }

    /// Decrease available vector
    pub fn dec_available(&mut self, res_id: usize, cnt: usize) {
        self.available[res_id] -= cnt;
    }

    /// Push cnt into available vector
    pub fn push_available(&mut self, cnt: usize) {
        self.available.push(cnt);
    }

    /// Set available vector
    pub fn set_available(&mut self, res_id: usize, cnt: usize) {
        self.available[res_id] = cnt;
    }

    /// Increase allocation matrix
    pub fn inc_allocation(&mut self, tid: usize, res_id: usize, cnt: usize) {
        self.allocation
            .entry(tid)
            .and_modify(|res_alloc| {
                res_alloc
                    .entry(res_id)
                    .and_modify(|alloc| *alloc += cnt)
                    .or_insert(1);
            })
            .or_insert(BTreeMap::from([(res_id, cnt)]));
    }

    /// Decrease allocation matrix
    pub fn dec_allocation(&mut self, tid: usize, res_id: usize, cnt: usize) {
        self.allocation.entry(tid).and_modify(|res_alloc| {
            res_alloc.entry(res_id).and_modify(|alloc| *alloc -= cnt);
            if let Some(alloc) = res_alloc.get(&res_id) {
                if *alloc <= 0 {
                    res_alloc.remove(&res_id);
                }
            }
        });
        if let Some(res_alloc) = self.allocation.get(&tid) {
            if res_alloc.is_empty() {
                self.allocation.remove(&tid);
            }
        }
    }

    /// Increase need matrix
    pub fn inc_need(&mut self, tid: usize, res_id: usize, cnt: usize) {
        self.need
            .entry(tid)
            .and_modify(|res_need| {
                res_need
                    .entry(res_id)
                    .and_modify(|need| *need += cnt)
                    .or_insert(1);
            })
            .or_insert(BTreeMap::from([(res_id, cnt)]));
    }

    /// Decrease need matrix
    pub fn dec_need(&mut self, tid: usize, res_id: usize, cnt: usize) {
        self.need.entry(tid).and_modify(|res_need| {
            res_need.entry(res_id).and_modify(|need| *need -= cnt);
            if let Some(need) = res_need.get(&res_id) {
                if *need <= 0 {
                    res_need.remove(&res_id);
                }
            }
        });
        if let Some(res_need) = self.need.get(&tid) {
            if res_need.is_empty() {
                self.need.remove(&tid);
            }
        }
    }

    /// Alloc resource
    pub fn alloc_res(&mut self, tid: usize, res_id: usize, cnt: usize) {
        self.dec_available(res_id, cnt);
        self.inc_allocation(tid, res_id, cnt);
        self.dec_need(tid, res_id, cnt);
    }

    /// Dealloc resource
    pub fn dealloc_res(&mut self, tid: usize, res_id: usize, cnt: usize) {
        self.dec_allocation(tid, res_id, cnt);
        self.inc_available(res_id, cnt);
    }

    /// Check if the status is safe
    // pub fn check(&mut self, tid: usize, res_id: usize) -> bool {
    pub fn check(&mut self) -> bool {
        let mut work = self.available.clone();
        let mut unfinished = BTreeSet::<usize>::new();
        for task_id in self.allocation.keys() {
            unfinished.insert(*task_id);
        }
        for task_id in self.need.keys() {
            unfinished.insert(*task_id);
        }
        loop {
            let mut t = None;
            // step 2
            for unfinished_tid in unfinished.iter() {
                let mut flag = true;
                if let Some(res_need) = self.need.get(unfinished_tid) {
                    for (sid, need) in res_need.iter() {
                        if *need > work[*sid] {
                            flag = false;
                            break;
                        }
                    }
                }
                if flag {
                    t = Some(unfinished_tid.clone());
                    break;
                }
            }
            if let Some(t) = t {
                // step 3
                if let Some(res_alloc) = self.allocation.get(&t) {
                    for (sid, alloc) in res_alloc.iter() {
                        work[*sid] += *alloc;
                    }
                }
                unfinished.remove(&t);
            } else {
                // step 4
                if unfinished.is_empty() {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}
