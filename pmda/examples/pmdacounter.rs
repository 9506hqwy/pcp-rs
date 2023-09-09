use once_cell::sync::Lazy;
use pmda::*;
use pmda_sys::*;
use std::ffi::NulError;
use std::sync::Mutex;

const DOMAIN: u32 = 450;

static COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn metric_up1() -> pmdaMetric {
    pmdaMetric {
        m_desc: pmDesc {
            pmid: pm_id(0, 0),
            type_: PM_TYPE_U64 as i32,
            indom: PM_INDOM_NULL,
            sem: PM_SEM_INSTANT as i32,
            units: pmUnits::default(),
        },
        ..Default::default()
    }
}

fn metric_up2() -> pmdaMetric {
    pmdaMetric {
        m_desc: pmDesc {
            pmid: pm_id(0, 1),
            type_: PM_TYPE_U64 as i32,
            indom: PM_INDOM_NULL,
            sem: PM_SEM_INSTANT as i32,
            units: pmUnits::default(),
        },
        ..Default::default()
    }
}

fn count_up1(atom: &mut AtomValue) -> i32 {
    let mut cnt = COUNTER.lock().unwrap();
    *cnt += 1;

    atom.set_u64(*cnt);
    0
}

fn count_up2(atom: &mut AtomValue) -> i32 {
    let mut cnt = COUNTER.lock().unwrap();
    *cnt += 2;

    atom.set_u64(*cnt);
    0
}

extern "C" fn counter_fetch(metric: *mut pmdaMetric, inst: u32, atom: *mut pmAtomValue) -> i32 {
    if inst != PM_IN_NULL {
        return PM_ERR_INST;
    }

    let metric = Metric::new(metric);

    if metric.desc().id().cluster() != 0 {
        return PM_ERR_PMID;
    }

    let mut atom = AtomValue::new(atom);

    match metric.desc().id().item() {
        0 => count_up1(&mut atom),
        1 => count_up2(&mut atom),
        _ => PM_ERR_PMID,
    }
}

fn main() -> Result<(), NulError> {
    let pmda = Pmda::new("counter")?;
    pmda.init();

    let mut dispatch = Interface::default();
    pmda.set_daemon(&mut dispatch, DOMAIN);

    let mut long_options = vec![
        pmdaopt_header(),
        pmdaopt_debug(),
        pmdaopt_domain(),
        pmdaopt_helptext(),
        pmdaopt_inet(),
        pmdaopt_logfile(),
        pmdaopt_pipe(),
        pmdaopt_unix(),
        pmdaopt_username(),
        pmdaopt_end(),
    ];

    let mut options = pmda_options(pmdaopt(), &mut long_options);

    dispatch.get_options(&mut options);
    if options.errors != 0 {
        pmda_print_usage(&mut options);
        return Ok(());
    }

    dispatch.open_log();

    pmda.set_uid();

    dispatch.set_fetch_callback(Some(counter_fetch));

    let mut metrics = vec![metric_up1(), metric_up2()];

    dispatch.init(&mut metrics);

    dispatch.connect();

    dispatch.main();

    Ok(())
}
