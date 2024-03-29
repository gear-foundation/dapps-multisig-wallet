use gstd::Encode;
use gtest::{Program, System};
use multisig_wallet_io::*;

const USERS: &[u64] = &[3, 4, 5, 6];

fn common_init<'a>(sys: &'a System, users: &[u64], required: u32) -> Program<'a> {
    sys.init_logger();

    let wallet = Program::current(sys);

    wallet.send(
        USERS[0],
        MWInitConfig {
            owners: users.iter().copied().map(|x| x.into()).collect(),
            required,
        },
    );

    wallet
}

#[test]
fn common() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..1], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(USERS[1].into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(
        USERS[1],
        MWAction::SubmitTransaction {
            destination: USERS[2].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));
}

#[test]
fn try_to_send_directly() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..1], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::AddOwner(USERS[1].into()).encode(),
        1_000_000_000,
    );

    assert!(res.main_failed());
}

#[test]
fn try_to_add_existing_owner() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..2], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(USERS[1].into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    assert!(res.others_failed());
}

#[test]
fn try_to_add_the_same_owner_twice() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..1], 1);

    sys.mint_to(USERS[0], 2_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(USERS[1].into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(USERS[1].into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    assert!(res.others_failed());
}

#[test]
fn try_to_add_to_max_owners() {
    let sys = System::new();
    let max: [u64; 50] = (3..=52).collect::<Vec<_>>().try_into().unwrap();
    let wallet = common_init(&sys, &max, 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(53.into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    assert!(res.others_failed());
}

#[test]
fn remove_than_add() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    sys.mint_to(USERS[0], 2_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::RemoveOwner(USERS[1].into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(USERS[1].into()).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(
        USERS[1],
        MWAction::SubmitTransaction {
            destination: USERS[2].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 2.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));
}
