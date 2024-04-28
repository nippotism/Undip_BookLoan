#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Loan {
    id: u64,
    book_title: String,
    borrower: String,
    borrowed_at: u64,
    due_date: u64,
}

impl Storable for Loan {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Loan {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, Loan, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct LoanPayload {
    book_title: String,
    borrower: String,
    due_date: u64,
}

#[ic_cdk::query]
fn get_loan(id: u64) -> Result<Loan, Error> {
    match _get_loan(&id) {
        Some(loan) => Ok(loan),
        None => Err(Error::NotFound {
            msg: format!("a loan with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_loan(loan: LoanPayload) -> Option<Loan> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let loan = Loan {
        id,
        book_title: loan.book_title,
        borrower: loan.borrower,
        borrowed_at: time(),
        due_date: loan.due_date,
    };
    do_insert(&loan);
    Some(loan)
}

#[ic_cdk::update]
fn update_loan(id: u64, payload: LoanPayload) -> Result<Loan, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut loan) => {
            loan.book_title = payload.book_title;
            loan.borrower = payload.borrower;
            loan.due_date = payload.due_date;
            do_insert(&loan);
            Ok(loan)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a loan with id={}. loan not found",
                id
            ),
        }),
    }
}

fn do_insert(loan: &Loan) {
    STORAGE.with(|service| service.borrow_mut().insert(loan.id, loan.clone()));
}

#[ic_cdk::update]
fn delete_loan(id: u64) -> Result<Loan, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(loan) => Ok(loan),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a loan with id={}. loan not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

fn _get_loan(id: &u64) -> Option<Loan> {
    STORAGE.with(|service| service.borrow().get(id))
}

ic_cdk::export_candid!();