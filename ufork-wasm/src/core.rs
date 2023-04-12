// uFork virtual CPU core

use alloc::boxed::Box;

use crate::*;
use crate::device::*;

const BNK_INI: Raw          = BNK_0;
//const BNK_INI: Raw          = BNK_1;

pub const ROM_BASE_OFS: usize = 16;  // ROM offsets below this value are reserved

pub const START: Any        = Any { raw: 16 };

pub const MEMORY: Any       = Any { raw: MUT_RAW | BNK_INI | 0 };
pub const DDEQUE: Any       = Any { raw: MUT_RAW | BNK_INI | 1 };
pub const BLOB_DEV: Any     = Any { raw: OPQ_RAW | MUT_RAW | BNK_INI | 2 };
pub const CLOCK_DEV: Any    = Any { raw: OPQ_RAW | MUT_RAW | BNK_INI | 3 };
pub const IO_DEV: Any       = Any { raw: OPQ_RAW | MUT_RAW | BNK_INI | 4 };
pub const SPONSOR: Any      = Any { raw: MUT_RAW | BNK_INI | 5 };

pub const RAM_BASE_OFS: usize = 6;  // RAM offsets below this value are reserved

// core limits
const QUAD_ROM_MAX: usize = 1<<10;  // 1K quad-cells of ROM
const QUAD_RAM_MAX: usize = 1<<8;   // 256 quad-cells of RAM
//const BLOB_RAM_MAX: usize = 64;     // 64 octets of Blob RAM (for testing)
const BLOB_RAM_MAX: usize = 1<<8;   // 256 octets of Blob RAM (for testing)
//const BLOB_RAM_MAX: usize = 1<<10;  // 1K octets of Blob RAM
//const BLOB_RAM_MAX: usize = 1<<12;  // 4K octets of Blob RAM
//const BLOB_RAM_MAX: usize = 1<<16;  // 64K octets of Blob RAM
const DEVICE_MAX:   usize = 3;      // number of Core devices

pub struct Core {
    quad_rom:   [Quad; QUAD_ROM_MAX],
    quad_ram0:  [Quad; QUAD_RAM_MAX],
    quad_ram1:  [Quad; QUAD_RAM_MAX],
    blob_ram:   [u8; BLOB_RAM_MAX],
    device:     [Option<Box<dyn Device>>; DEVICE_MAX],
    rom_top:    Any,
}

impl Core {
    pub fn new() -> Core {

        /*
         * Read-Only Memory (ROM) image (read/execute)
         */
        let mut quad_rom = [
            Quad::empty_t();
            QUAD_ROM_MAX
        ];

        quad_rom[UNDEF.ofs()]       = Quad::literal_t();
        quad_rom[NIL.ofs()]         = Quad::literal_t();
        quad_rom[FALSE.ofs()]       = Quad::literal_t();
        quad_rom[TRUE.ofs()]        = Quad::literal_t();
        quad_rom[UNIT.ofs()]        = Quad::literal_t();

        quad_rom[EMPTY_DQ.ofs()]    = Quad::pair_t(NIL, NIL);

        quad_rom[TYPE_T.ofs()]      = Quad::type_t();
        quad_rom[FIXNUM_T.ofs()]    = Quad::type_t();
        quad_rom[ACTOR_T.ofs()]     = Quad::type_t();
        quad_rom[PROXY_T.ofs()]     = Quad::type_t();
        quad_rom[STUB_T.ofs()]      = Quad::type_t();
        quad_rom[INSTR_T.ofs()]     = Quad::type_t();
        quad_rom[PAIR_T.ofs()]      = Quad::type_t();
        quad_rom[DICT_T.ofs()]      = Quad::type_t();
        quad_rom[GC_FWD_T.ofs()]    = Quad::type_t();
        quad_rom[FREE_T.ofs()]      = Quad::type_t();

pub const SINK_BEH: Any     = Any { raw: 16 };  // alias for no-op behavior
pub const COMMIT: Any       = Any { raw: 16 };
        quad_rom[COMMIT.ofs()]      = Quad::vm_end_commit();
pub const SEND_0: Any       = Any { raw: 17 };
        quad_rom[SEND_0.ofs()]      = Quad::vm_send(ZERO, COMMIT);
pub const CUST_SEND: Any    = Any { raw: 18 };
        quad_rom[CUST_SEND.ofs()]   = Quad::vm_msg(PLUS_1, SEND_0);
pub const RV_SELF: Any      = Any { raw: 19 };
        quad_rom[RV_SELF.ofs()]     = Quad::vm_my_self(CUST_SEND);
pub const RV_UNDEF: Any     = Any { raw: 20 };
        quad_rom[RV_UNDEF.ofs()]    = Quad::vm_push(UNDEF, CUST_SEND);
pub const RV_NIL: Any       = Any { raw: 21 };
        quad_rom[RV_NIL.ofs()]      = Quad::vm_push(NIL, CUST_SEND);
pub const RV_FALSE: Any     = Any { raw: 22 };
        quad_rom[RV_FALSE.ofs()]    = Quad::vm_push(FALSE, CUST_SEND);
pub const RV_TRUE: Any      = Any { raw: 23 };
        quad_rom[RV_TRUE.ofs()]     = Quad::vm_push(TRUE, CUST_SEND);
pub const RV_UNIT: Any      = Any { raw: 24 };
        quad_rom[RV_UNIT.ofs()]     = Quad::vm_push(UNIT, CUST_SEND);
pub const RV_ZERO: Any      = Any { raw: 25 };
        quad_rom[RV_ZERO.ofs()]     = Quad::vm_push(ZERO, CUST_SEND);
pub const RV_ONE: Any       = Any { raw: 26 };
        quad_rom[RV_ONE.ofs()]      = Quad::vm_push(PLUS_1, CUST_SEND);
pub const RESEND: Any       = Any { raw: 27 };
        quad_rom[RESEND.ofs()+0]    = Quad::vm_msg(ZERO, Any::rom(RESEND.ofs()+1));
        quad_rom[RESEND.ofs()+1]    = Quad::vm_my_self(SEND_0);
pub const RELEASE: Any      = Any { raw: 29 };
        quad_rom[RELEASE.ofs()]     = Quad::vm_end_release();
pub const RELEASE_0: Any    = Any { raw: 30 };
        quad_rom[RELEASE_0.ofs()]   = Quad::vm_send(ZERO, RELEASE);
pub const STOP: Any         = Any { raw: 31 };
        quad_rom[STOP.ofs()]        = Quad::vm_end_stop();
pub const ABORT: Any        = Any { raw: 32 };
        quad_rom[ABORT.ofs()+0]     = Quad::vm_push(UNDEF, Any::rom(ABORT.ofs()+1));  // reason=#?
        quad_rom[ABORT.ofs()+1]     = Quad::vm_end_abort();

pub const MEMO_OFS: usize = 34;
pub const _MEMO_BEH: Any = Any { raw: MEMO_OFS as Raw };
        /*
        (define memo-beh
            (lambda (value)
                (BEH (cust . _)
                    (SEND cust value) )))
        */
        // stack: value
        quad_rom[MEMO_OFS+0]        = Quad::vm_dup(PLUS_1, CUST_SEND);  // value value

pub const FWD_OFS: usize = MEMO_OFS+1;
pub const _FWD_BEH: Any = Any { raw: FWD_OFS as Raw };
        /*
        (define fwd-beh
            (lambda (rcvr)
                (BEH msg
                    (SEND rcvr msg) )))
        */
        // stack: rcvr
        quad_rom[FWD_OFS+0]         = Quad::vm_msg(ZERO, Any::rom(FWD_OFS+1));  // rcvr msg
        quad_rom[FWD_OFS+1]         = Quad::vm_pick(PLUS_2, SEND_0);  // rcvr msg rcvr

pub const ONCE_OFS: usize = FWD_OFS+2;
pub const _ONCE_BEH: Any = Any { raw: ONCE_OFS as Raw };
        /*
        (define once-beh
            (lambda (rcvr)
                (BEH msg
                    (BECOME sink-beh)
                    (SEND rcvr msg) )))
        */
        // stack: rcvr
        quad_rom[ONCE_OFS+0]        = Quad::vm_push(SINK_BEH, Any::rom(ONCE_OFS+1));  // rcvr sink-beh
        quad_rom[ONCE_OFS+1]        = Quad::vm_beh(ZERO, _FWD_BEH);  // rcvr

pub const LABEL_OFS: usize = ONCE_OFS+2;
pub const _LABEL_BEH: Any = Any { raw: LABEL_OFS as Raw };
        /*
        (define label-beh
            (lambda (rcvr label)
                (BEH msg
                    (SEND rcvr (cons label msg)) )))
        */
        // stack: rcvr label
        quad_rom[LABEL_OFS+0]       = Quad::vm_msg(ZERO, Any::rom(LABEL_OFS+1));  // rcvr label msg
        quad_rom[LABEL_OFS+1]       = Quad::vm_pick(PLUS_2, Any::rom(LABEL_OFS+2));  // rcvr label msg label
        quad_rom[LABEL_OFS+2]       = Quad::vm_pair(PLUS_1, Any::rom(LABEL_OFS+3));  // rcvr label (label . msg)
        quad_rom[LABEL_OFS+3]       = Quad::vm_pick(PLUS_3, SEND_0);  // rcvr label (label . msg) rcvr

pub const TAG_OFS: usize = LABEL_OFS+4;
pub const _TAG_BEH: Any = Any { raw: TAG_OFS as Raw };
        /*
        (define tag-beh
            (lambda (rcvr)
                (BEH msg
                    (SEND rcvr (cons SELF msg)) )))
        */
        // stack: rcvr
        quad_rom[TAG_OFS+0]         = Quad::vm_my_self(_LABEL_BEH);  // rcvr SELF

pub const ONCE_TAG_OFS: usize = TAG_OFS+1;
pub const ONCE_TAG_BEH: Any = Any { raw: ONCE_TAG_OFS as Raw };
        /*
        (define once-tag-beh  ;; FIXME: find a better name for this?
            (lambda (rcvr)
                (BEH msg
                    (BECOME sink-beh)
                    (SEND rcvr (cons SELF msg)) )))
        */
        // stack: rcvr
        quad_rom[ONCE_TAG_OFS+0]    = Quad::vm_push(SINK_BEH, Any::rom(ONCE_TAG_OFS+1));  // rcvr sink-beh
        quad_rom[ONCE_TAG_OFS+1]    = Quad::vm_beh(ZERO, _TAG_BEH);  // rcvr

pub const WRAP_OFS: usize = ONCE_TAG_OFS+2;
pub const _WRAP_BEH: Any = Any { raw: WRAP_OFS as Raw };
        /*
        (define wrap-beh
            (lambda (rcvr)
                (BEH msg
                    (SEND rcvr (list msg)) )))
        */
        // stack: rcvr
        quad_rom[WRAP_OFS+0]        = Quad::vm_msg(ZERO, Any::rom(WRAP_OFS+1));  // rcvr msg
        quad_rom[WRAP_OFS+1]        = Quad::vm_pick(PLUS_2, Any::rom(WRAP_OFS+2));  // rcvr msg rcvr
        quad_rom[WRAP_OFS+2]        = Quad::vm_send(PLUS_1, COMMIT);  // rcvr

pub const UNWRAP_OFS: usize = WRAP_OFS+3;
pub const _UNWRAP_BEH: Any = Any { raw: UNWRAP_OFS as Raw };
        /*
        (define unwrap-beh
            (lambda (rcvr)
                (BEH (msg)
                    (SEND rcvr msg) )))
        */
        // stack: rcvr
        quad_rom[UNWRAP_OFS+0]      = Quad::vm_msg(PLUS_1, Any::rom(UNWRAP_OFS+1));  // rcvr msg
        quad_rom[UNWRAP_OFS+1]      = Quad::vm_pick(PLUS_2, SEND_0);  // rcvr msg rcvr

pub const FUTURE_OFS: usize = UNWRAP_OFS+2;
pub const _FUTURE_BEH: Any = Any { raw: FUTURE_OFS as Raw };
        /*
        (define future-beh
            (lambda (rcap wcap)
                (BEH (tag . arg)
                    (cond
                        ((eq? tag rcap)
                            (BECOME (wait-beh rcap wcap (list arg))))
                        ((eq? tag wcap)
                            (BECOME (value-beh rcap arg))) ))))
        */
        // stack: rcap wcap
        quad_rom[FUTURE_OFS+0]      = Quad::vm_msg(PLUS_1, Any::rom(FUTURE_OFS+1));  // rcap wcap tag
        quad_rom[FUTURE_OFS+1]      = Quad::vm_pick(PLUS_3, Any::rom(FUTURE_OFS+2));  // rcap wcap tag rcap
        quad_rom[FUTURE_OFS+2]      = Quad::vm_cmp_eq(Any::rom(FUTURE_OFS+3));  // rcap wcap tag==rcap
        quad_rom[FUTURE_OFS+3]      = Quad::vm_if(Any::rom(FUTURE_OFS+4), Any::rom(FUTURE_OFS+9));  // rcap wcap

        quad_rom[FUTURE_OFS+4]      = Quad::vm_push(NIL, Any::rom(FUTURE_OFS+5));  // rcap wcap ()
        quad_rom[FUTURE_OFS+5]      = Quad::vm_msg(MINUS_1, Any::rom(FUTURE_OFS+6));  // rcap wcap () arg
        quad_rom[FUTURE_OFS+6]      = Quad::vm_pair(PLUS_1, Any::rom(FUTURE_OFS+7));  // rcap wcap (arg)
        quad_rom[FUTURE_OFS+7]      = Quad::vm_push(_WAIT_BEH, Any::rom(FUTURE_OFS+8));  // rcap wcap (arg) wait-beh
        quad_rom[FUTURE_OFS+8]      = Quad::vm_beh(PLUS_3, COMMIT);  // wait-beh[rcap wcap (arg)]

        quad_rom[FUTURE_OFS+9]      = Quad::vm_msg(PLUS_1, Any::rom(FUTURE_OFS+10));  // rcap wcap tag
        quad_rom[FUTURE_OFS+10]     = Quad::vm_pick(PLUS_2, Any::rom(FUTURE_OFS+11));  // rcap wcap tag wcap
        quad_rom[FUTURE_OFS+11]     = Quad::vm_cmp_eq(Any::rom(FUTURE_OFS+12));  // rcap wcap tag==wcap
        quad_rom[FUTURE_OFS+12]     = Quad::vm_if(Any::rom(FUTURE_OFS+13), ABORT);  // rcap wcap

        quad_rom[FUTURE_OFS+13]     = Quad::vm_drop(PLUS_1, Any::rom(FUTURE_OFS+14));  // rcap
        quad_rom[FUTURE_OFS+14]     = Quad::vm_msg(MINUS_1, Any::rom(FUTURE_OFS+15));  // rcap value=arg
        quad_rom[FUTURE_OFS+15]     = Quad::vm_push(_VALUE_BEH, Any::rom(FUTURE_OFS+16));  // rcap value=arg value-beh
        quad_rom[FUTURE_OFS+16]     = Quad::vm_beh(PLUS_2, COMMIT);  // value-beh[rcap value]

pub const WAIT_OFS: usize = FUTURE_OFS+17;
pub const _WAIT_BEH: Any = Any { raw: WAIT_OFS as Raw };
        /*
        (define wait-beh
            (lambda (rcap wcap waiting)
                (BEH (tag . arg)
                    (cond
                        ((eq? tag rcap)
                            (BECOME (wait-beh rcap wcap (cons arg waiting))))
                        ((eq? tag wcap)
                            (send-to-all waiting arg)
                            (BECOME (value-beh rcap arg))) ))))
        */
        // stack: rcap wcap waiting
        quad_rom[WAIT_OFS+0]        = Quad::vm_msg(PLUS_1, Any::rom(WAIT_OFS+1));  // rcap wcap waiting tag
        quad_rom[WAIT_OFS+1]        = Quad::vm_pick(PLUS_4, Any::rom(WAIT_OFS+2));  // rcap wcap waiting tag rcap
        quad_rom[WAIT_OFS+2]        = Quad::vm_cmp_eq(Any::rom(WAIT_OFS+3));  // rcap wcap waiting tag==rcap
        quad_rom[WAIT_OFS+3]        = Quad::vm_if(Any::rom(WAIT_OFS+4), Any::rom(WAIT_OFS+8));  // rcap wcap waiting

        quad_rom[WAIT_OFS+4]        = Quad::vm_msg(MINUS_1, Any::rom(WAIT_OFS+5));  // rcap wcap waiting arg
        quad_rom[WAIT_OFS+5]        = Quad::vm_pair(PLUS_1, Any::rom(WAIT_OFS+6));  // rcap wcap (arg . waiting)
        quad_rom[WAIT_OFS+6]        = Quad::vm_push(_WAIT_BEH, Any::rom(WAIT_OFS+7));  // rcap wcap (arg . waiting) wait-beh
        quad_rom[WAIT_OFS+7]        = Quad::vm_beh(PLUS_3, COMMIT);  // wait-beh[rcap wcap (arg . waiting)]

        quad_rom[WAIT_OFS+8]        = Quad::vm_msg(PLUS_1, Any::rom(WAIT_OFS+9));  // rcap wcap waiting tag
        quad_rom[WAIT_OFS+9]        = Quad::vm_pick(PLUS_2, Any::rom(WAIT_OFS+10));  // rcap wcap waiting tag wcap
        quad_rom[WAIT_OFS+10]       = Quad::vm_cmp_eq(Any::rom(WAIT_OFS+11));  // rcap wcap waiting tag==wcap
        quad_rom[WAIT_OFS+11]       = Quad::vm_if(Any::rom(WAIT_OFS+12), ABORT);  // rcap wcap waiting

        quad_rom[WAIT_OFS+12]       = Quad::vm_dup(PLUS_1, Any::rom(WAIT_OFS+13));  // rcap wcap waiting waiting
        quad_rom[WAIT_OFS+13]       = Quad::vm_typeq(PAIR_T, Any::rom(WAIT_OFS+14));  // rcap wcap waiting is_pair(waiting)
        quad_rom[WAIT_OFS+14]       = Quad::vm_if(Any::rom(WAIT_OFS+15), Any::rom(WAIT_OFS+19));  // rcap wcap waiting
        quad_rom[WAIT_OFS+15]       = Quad::vm_part(PLUS_1, Any::rom(WAIT_OFS+16));  // rcap wcap rest first
        quad_rom[WAIT_OFS+16]       = Quad::vm_msg(MINUS_1, Any::rom(WAIT_OFS+17));  // rcap wcap rest first value=arg
        quad_rom[WAIT_OFS+17]       = Quad::vm_roll(PLUS_2, Any::rom(WAIT_OFS+18));  // rcap wcap rest value=arg first
        quad_rom[WAIT_OFS+18]       = Quad::vm_send(ZERO, Any::rom(WAIT_OFS+12));  // rcap wcap rest

        quad_rom[WAIT_OFS+19]       = Quad::vm_drop(PLUS_2, Any::rom(WAIT_OFS+20));  // rcap
        quad_rom[WAIT_OFS+20]       = Quad::vm_msg(MINUS_1, Any::rom(WAIT_OFS+21));  // rcap value=arg
        quad_rom[WAIT_OFS+21]       = Quad::vm_push(_VALUE_BEH, Any::rom(WAIT_OFS+22));  // rcap value=arg value-beh
        quad_rom[WAIT_OFS+22]       = Quad::vm_beh(PLUS_2, COMMIT);  // value-beh[rcap value]

pub const VALUE_OFS: usize = WAIT_OFS+23;
pub const _VALUE_BEH: Any = Any { raw: VALUE_OFS as Raw };
        /*
        (define value-beh
            (lambda (rcap value)
                (BEH (tag . arg)
                    (cond
                        ((eq? tag rcap)
                            (SEND arg value))) )))
        */
        // stack: rcap value
        quad_rom[VALUE_OFS+0]       = Quad::vm_msg(PLUS_1, Any::rom(VALUE_OFS+1));  // rcap value tag
        quad_rom[VALUE_OFS+1]       = Quad::vm_pick(PLUS_3, Any::rom(VALUE_OFS+2));  // rcap value tag rcap
        quad_rom[VALUE_OFS+2]       = Quad::vm_cmp_eq(Any::rom(VALUE_OFS+3));  // rcap value tag==rcap
        quad_rom[VALUE_OFS+3]       = Quad::vm_if(Any::rom(VALUE_OFS+4), COMMIT);  // rcap value
        quad_rom[VALUE_OFS+4]       = Quad::vm_pick(PLUS_1, Any::rom(VALUE_OFS+5));  // rcap value value
        quad_rom[VALUE_OFS+5]       = Quad::vm_msg(MINUS_1, SEND_0);  // rcap value value cust=arg

pub const SERIAL_OFS: usize = VALUE_OFS+6;
pub const _SERIAL_BEH: Any = Any { raw: SERIAL_OFS as Raw };
        /*
        (define serial-beh
            (lambda (svc)
                (BEH (cust . req)
                    (define tag (CREATE (once-tag-beh SELF)))
                    (SEND svc (tag . req))
                    (BECOME (busy-beh svc cust tag (deque-new))) )))
        */
        // stack: svc
        quad_rom[SERIAL_OFS+0]      = Quad::vm_msg(PLUS_1, Any::rom(SERIAL_OFS+1));  // svc cust
        quad_rom[SERIAL_OFS+1]      = Quad::vm_my_self(Any::rom(SERIAL_OFS+2));  // svc cust SELF
        quad_rom[SERIAL_OFS+2]      = Quad::vm_push(ONCE_TAG_BEH, Any::rom(SERIAL_OFS+3));  // svc cust SELF once-tag-beh
        quad_rom[SERIAL_OFS+3]      = Quad::vm_new(PLUS_1, Any::rom(SERIAL_OFS+4));  // svc cust tag=once-tag-beh[SELF]

        quad_rom[SERIAL_OFS+4]      = Quad::vm_msg(MINUS_1, Any::rom(SERIAL_OFS+5));  // svc cust tag req
        quad_rom[SERIAL_OFS+5]      = Quad::vm_pick(PLUS_2, Any::rom(SERIAL_OFS+6));  // svc cust tag req tag
        quad_rom[SERIAL_OFS+6]      = Quad::vm_pair(PLUS_1, Any::rom(SERIAL_OFS+7));  // svc cust tag (tag . req)
        quad_rom[SERIAL_OFS+7]      = Quad::vm_pick(PLUS_4, Any::rom(SERIAL_OFS+8));  // svc cust tag (tag . req) svc
        quad_rom[SERIAL_OFS+8]      = Quad::vm_send(ZERO, Any::rom(SERIAL_OFS+9));  // svc cust tag

        quad_rom[SERIAL_OFS+9]      = Quad::vm_deque_new(Any::rom(SERIAL_OFS+10));  // svc cust tag pending
        quad_rom[SERIAL_OFS+10]     = Quad::vm_push(_BUSY_BEH, Any::rom(SERIAL_OFS+11));  // svc cust tag pending busy-beh
        quad_rom[SERIAL_OFS+11]     = Quad::vm_beh(PLUS_4, COMMIT);  // busy-beh[svc cust tag pending]

pub const BUSY_OFS: usize = SERIAL_OFS+12;
pub const _BUSY_BEH: Any = Any { raw: BUSY_OFS as Raw };
        /*
        (define busy-beh
            (lambda (svc cust tag pending)
                (BEH (cust0 . req0)
                    (cond
                        ((eq? cust0 tag)
                            (SEND cust req0)
                            (define (next pending1) (deque-pop pending))
                            (cond
                                ((eq? next #?)
                                    (BECOME (serial-beh svc)))  ; return to "ready" state
                                (#t
                                    (define (cust1 . req1) next)
                                    (define tag1 (CREATE (once-tag-beh SELF)))
                                    (SEND svc (tag1 . req1))
                                    (BECOME (busy-beh svc cust1 tag1 pending1)) )))
                        (#t
                            (define pending1 (deque-put pending (cons cust0 req0)))
                            (BECOME (busy-beh svc cust tag pending1))) ))))
                    )))
        */
        // stack: svc cust tag pending
        quad_rom[BUSY_OFS+0]        = Quad::vm_msg(PLUS_1, Any::rom(BUSY_OFS+1));  // svc cust tag pending cust0
        quad_rom[BUSY_OFS+1]        = Quad::vm_pick(PLUS_3, Any::rom(BUSY_OFS+2));  // svc cust tag pending cust0 tag
        quad_rom[BUSY_OFS+2]        = Quad::vm_cmp_eq(Any::rom(BUSY_OFS+3));  // svc cust tag pending cust0==tag
        quad_rom[BUSY_OFS+3]        = Quad::vm_if(Any::rom(BUSY_OFS+4), Any::rom(BUSY_OFS+28));  // svc cust tag pending

        quad_rom[BUSY_OFS+4]        = Quad::vm_msg(MINUS_1, Any::rom(BUSY_OFS+5));  // svc cust tag pending req0
        quad_rom[BUSY_OFS+5]        = Quad::vm_roll(PLUS_4, Any::rom(BUSY_OFS+6));  // svc tag pending req0 cust
        quad_rom[BUSY_OFS+6]        = Quad::vm_send(ZERO, Any::rom(BUSY_OFS+7));  // svc tag pending
        quad_rom[BUSY_OFS+7]        = Quad::vm_deque_pop(Any::rom(BUSY_OFS+8));  // svc tag pending1 next
        quad_rom[BUSY_OFS+8]        = Quad::vm_dup(PLUS_1, Any::rom(BUSY_OFS+9));  // svc tag pending1 next next
        quad_rom[BUSY_OFS+9]        = Quad::vm_eq(UNDEF, Any::rom(BUSY_OFS+10));  // svc tag pending1 next next==#?
        quad_rom[BUSY_OFS+10]       = Quad::vm_if(Any::rom(BUSY_OFS+11), Any::rom(BUSY_OFS+14));  // svc tag pending1 next

        quad_rom[BUSY_OFS+11]       = Quad::vm_drop(PLUS_3, Any::rom(BUSY_OFS+12));  // svc
        quad_rom[BUSY_OFS+12]       = Quad::vm_push(_SERIAL_BEH, Any::rom(BUSY_OFS+13));  // svc serial-beh
        quad_rom[BUSY_OFS+13]       = Quad::vm_beh(PLUS_1, COMMIT);  // serial-beh[svc]

        quad_rom[BUSY_OFS+14]       = Quad::vm_part(PLUS_1, Any::rom(BUSY_OFS+15));  // svc tag pending1 req1 cust1
        quad_rom[BUSY_OFS+15]       = Quad::vm_my_self(Any::rom(BUSY_OFS+16));  // svc tag pending1 req1 cust1 SELF
        quad_rom[BUSY_OFS+16]       = Quad::vm_push(ONCE_TAG_BEH, Any::rom(BUSY_OFS+17));  // svc tag pending1 req1 cust1 SELF once-tag-beh
        quad_rom[BUSY_OFS+17]       = Quad::vm_new(PLUS_1, Any::rom(BUSY_OFS+18));  // svc tag pending1 req1 cust1 tag1=once-tag-beh[SELF]
        quad_rom[BUSY_OFS+18]       = Quad::vm_roll(PLUS_3, Any::rom(BUSY_OFS+19));  // svc tag pending1 cust1 tag1 req1
        quad_rom[BUSY_OFS+19]       = Quad::vm_pick(PLUS_2, Any::rom(BUSY_OFS+20));  // svc tag pending1 cust1 tag1 req1 tag1
        quad_rom[BUSY_OFS+20]       = Quad::vm_pair(PLUS_1, Any::rom(BUSY_OFS+21));  // svc tag pending1 cust1 tag1 (tag1 . req1)
        quad_rom[BUSY_OFS+21]       = Quad::vm_pick(PLUS_6, Any::rom(BUSY_OFS+22));  // svc tag pending1 cust1 tag1 (tag1 . req1) svc
        quad_rom[BUSY_OFS+22]       = Quad::vm_send(ZERO, Any::rom(BUSY_OFS+23));  // svc tag pending1 cust1 tag1
        quad_rom[BUSY_OFS+23]       = Quad::vm_roll(PLUS_5, Any::rom(BUSY_OFS+24));  // tag pending1 cust1 tag1 svc
        quad_rom[BUSY_OFS+24]       = Quad::vm_roll(MINUS_3, Any::rom(BUSY_OFS+25));  // tag pending1 svc cust1 tag1
        quad_rom[BUSY_OFS+25]       = Quad::vm_roll(PLUS_4, Any::rom(BUSY_OFS+26));  // tag svc cust1 tag1 pending1

        quad_rom[BUSY_OFS+26]       = Quad::vm_push(_BUSY_BEH, Any::rom(BUSY_OFS+27));  // ... svc cust1 tag1 pending1 busy-beh
        quad_rom[BUSY_OFS+27]       = Quad::vm_beh(PLUS_4, COMMIT);  // busy-beh[svc cust1 tag1 pending1]

        quad_rom[BUSY_OFS+28]       = Quad::vm_msg(ZERO, Any::rom(BUSY_OFS+29));  // svc cust tag pending (cust0 . req0)
        quad_rom[BUSY_OFS+29]       = Quad::vm_deque_put(Any::rom(BUSY_OFS+26));  // svc cust tag pending1

pub const F_FIB_OFS: usize = BUSY_OFS+30;
pub const F_FIB_BEH: Any = Any { raw: F_FIB_OFS as Raw };
        /*
        (define fib                 ; O(n!) performance?
          (lambda (n)               ; msg: (cust n)
            (if (< n 2)
                n
                (+ (fib (- n 1)) (fib (- n 2))) )))
        */
        quad_rom[F_FIB_OFS+0]       = Quad::vm_msg(PLUS_2, Any::rom(F_FIB_OFS+1));  // n
        quad_rom[F_FIB_OFS+1]       = Quad::vm_dup(PLUS_1, Any::rom(F_FIB_OFS+2));  // n n
        quad_rom[F_FIB_OFS+2]       = Quad::vm_push(PLUS_2, Any::rom(F_FIB_OFS+3));  // n n 2
        quad_rom[F_FIB_OFS+3]       = Quad::vm_cmp_lt(Any::rom(F_FIB_OFS+4));  // n n<2
        quad_rom[F_FIB_OFS+4]       = Quad::vm_if(CUST_SEND, Any::rom(F_FIB_OFS+5));  // n

        quad_rom[F_FIB_OFS+5]       = Quad::vm_msg(PLUS_1, Any::rom(F_FIB_OFS+6));  // n cust
        quad_rom[F_FIB_OFS+6]       = Quad::vm_push(F_FIB_K, Any::rom(F_FIB_OFS+7));  // n cust fib-k
        quad_rom[F_FIB_OFS+7]       = Quad::vm_new(PLUS_1, Any::rom(F_FIB_OFS+8));  // n k=fib-k[cust]

        quad_rom[F_FIB_OFS+8]       = Quad::vm_pick(PLUS_2, Any::rom(F_FIB_OFS+9));  // n k n
        quad_rom[F_FIB_OFS+9]       = Quad::vm_push(PLUS_1, Any::rom(F_FIB_OFS+10));  // n k n 1
        quad_rom[F_FIB_OFS+10]      = Quad::vm_alu_sub(Any::rom(F_FIB_OFS+11));  // n k n-1
        quad_rom[F_FIB_OFS+11]      = Quad::vm_pick(PLUS_2, Any::rom(F_FIB_OFS+12));  // n k n-1 k
        //quad_rom[F_FIB_OFS+12]      = Quad::vm_my_self(Any::rom(F_FIB_OFS+14));  // n k n-1 k fib
        quad_rom[F_FIB_OFS+12]      = Quad::vm_push(F_FIB_BEH, Any::rom(F_FIB_OFS+13));  // n k n-1 k fib-beh
        quad_rom[F_FIB_OFS+13]      = Quad::vm_new(ZERO, Any::rom(F_FIB_OFS+14));  // n k n-1 k fib
        quad_rom[F_FIB_OFS+14]      = Quad::vm_send(PLUS_2, Any::rom(F_FIB_OFS+15));  // n k

        quad_rom[F_FIB_OFS+15]      = Quad::vm_roll(PLUS_2, Any::rom(F_FIB_OFS+16));  // k n
        quad_rom[F_FIB_OFS+16]      = Quad::vm_push(PLUS_2, Any::rom(F_FIB_OFS+17));  // k n 2
        quad_rom[F_FIB_OFS+17]      = Quad::vm_alu_sub(Any::rom(F_FIB_OFS+18));  // k n-2
        quad_rom[F_FIB_OFS+18]      = Quad::vm_roll(PLUS_2, Any::rom(F_FIB_OFS+19));  // n-2 k
        //quad_rom[F_FIB_OFS+19]      = Quad::vm_my_self(Any::rom(F_FIB_OFS+21));  // n-2 k fib
        quad_rom[F_FIB_OFS+19]      = Quad::vm_push(F_FIB_BEH, Any::rom(F_FIB_OFS+20));  // n-2 k fib-beh
        quad_rom[F_FIB_OFS+20]      = Quad::vm_new(ZERO, Any::rom(F_FIB_OFS+21));  // n-2 k fib
        quad_rom[F_FIB_OFS+21]      = Quad::vm_send(PLUS_2, COMMIT);  // --

pub const F_FIB_K: Any = Any { raw: (F_FIB_OFS+22) as Raw };
        // stack: cust
        quad_rom[F_FIB_OFS+22]      = Quad::vm_msg(ZERO, Any::rom(F_FIB_OFS+23));  // cust m
        quad_rom[F_FIB_OFS+23]      = Quad::vm_push(F_FIB_K2, Any::rom(F_FIB_OFS+24));  // cust m fib-k2
        quad_rom[F_FIB_OFS+24]      = Quad::vm_beh(PLUS_2, COMMIT);  // fib-k2[cust m]

pub const F_FIB_K2: Any = Any { raw: (F_FIB_OFS+25) as Raw };
        // stack: cust m
        quad_rom[F_FIB_OFS+25]      = Quad::vm_msg(ZERO, Any::rom(F_FIB_OFS+26));  // cust m n
        quad_rom[F_FIB_OFS+26]      = Quad::vm_alu_add(Any::rom(F_FIB_OFS+27));  // cust m+n
        quad_rom[F_FIB_OFS+27]      = Quad::vm_roll(PLUS_2, SEND_0);  // m+n cust

/*
(define COMMIT
    (vm-end-commit))
(define SEND-0  ; msg target
    (vm-send 0 COMMIT))
(define CUST-SEND  ; msg
    (vm-msg 1 SEND-0))
(define fib-k2  ; cust m
    (vm-msg 0  ; cust m n
      (vm-alu-add  ; cust m+n
        (vm-roll 2  ; m+n cust
          SEND-0))))
(define fib-k  ; cust
    (vm-msg 0  ; cust m
      (vm-push fib-k2  ; cust m fib-k2
        (vm-beh 2  ; (fib-k2 cust m)
          COMMIT))))
(define fib  ; (n)
    (CREATE  ; (cust n)
      (vm-msg 2  ; n
        (vm-dup 1  ; n n
          (vm-push 2  ; n n 2
            (vm-cmp-lt  ; n n<2
              (vm-if  ; n
                CUST-SEND
                (vm-msg 1  ; n cust
                  (vm-push fib-k  ; n cust fib-k
                    (vm-new 1  ; n k=(fib-k cust)
                      (vm-pick 2  ; n k n
                        (vm-push 1  ; n k n 1
                          (vm-alu-sub  ; n k n-1
                            (vm-pick 2  ; n k n-1 k
                              (vm-my-self  ; n k n-1 k fib
                                (vm-send 2  ; n k
                                  (vm-roll 2  ; k n
                                    (vm-push 2  ; k n 2
                                      (vm-alu-sub  ; k n-2
                                        (vm-roll 2  ; n-2 k
                                          (vm-my-self  ; n-2 k fib
                                            (vm-send 2  ; --
                                              COMMIT))))))
                                ))))))
                    )))
            )))))))
*/

pub const IS_EQ_OFS: usize = F_FIB_OFS+28;
pub const _IS_EQ_BEH: Any    = Any { raw: IS_EQ_OFS as Raw };
        /*
        (define is-eq-beh
            (lambda (expect)
                (BEH actual
                    (assert-eq expect actual) )))
        */
        // stack: expect
        quad_rom[IS_EQ_OFS+0]       = Quad::vm_dup(PLUS_1, Any::rom(IS_EQ_OFS+1));  // expect expect
        quad_rom[IS_EQ_OFS+1]       = Quad::vm_msg(ZERO, Any::rom(IS_EQ_OFS+2));  // expect expect actual
        quad_rom[IS_EQ_OFS+2]       = Quad::vm_cmp_eq(Any::rom(IS_EQ_OFS+3));  // expect (expect == actual)
        quad_rom[IS_EQ_OFS+3]       = Quad::vm_is_eq(TRUE, COMMIT);  // expect

        /* testcase: fib(6) => 8 */
pub const TEST_OFS: usize = IS_EQ_OFS+4;
pub const _TEST_BEH: Any    = Any { raw: TEST_OFS as Raw };
        quad_rom[TEST_OFS+0]        = Quad::vm_drop(PLUS_3, Any::rom(TEST_OFS+1));  // --
        quad_rom[TEST_OFS+1]        = Quad::vm_push(PLUS_6, Any::rom(TEST_OFS+2));  // 6
        quad_rom[TEST_OFS+2]        = Quad::vm_push(EQ_8_BEH, Any::rom(TEST_OFS+3));  // 6 eq-8-beh
        quad_rom[TEST_OFS+3]        = Quad::vm_new(ZERO, Any::rom(TEST_OFS+4));  // 6 eq-8
        //quad_rom[TEST_OFS+4]        = Quad::vm_push(F_FIB, Any::rom(TEST_OFS+6));  // 6 eq-8 f-fib
        quad_rom[TEST_OFS+4]        = Quad::vm_push(F_FIB_BEH, Any::rom(TEST_OFS+5));  // 6 eq-8 fib-beh
        quad_rom[TEST_OFS+5]        = Quad::vm_new(ZERO, Any::rom(TEST_OFS+6));  // 6 eq-8 fib
        quad_rom[TEST_OFS+6]        = Quad::vm_send(PLUS_2, COMMIT);  // --
pub const EQ_8_BEH: Any = Any { raw: (TEST_OFS+7) as Raw };
        quad_rom[TEST_OFS+7]        = Quad::vm_msg(ZERO, Any::rom(TEST_OFS+8));  // msg
        quad_rom[TEST_OFS+8]        = Quad::vm_is_eq(PLUS_8, COMMIT);  // assert_eq(8, msg)
        //quad_rom[TEST_OFS+8]        = Quad::vm_is_eq(PLUS_8, STOP);  // assert_eq(8, msg)
        //quad_rom[TEST_OFS+8]        = Quad::vm_is_eq(MINUS_1, COMMIT);  // assert_eq(-1, msg)  <-- this should fail!

        /* VM_DICT test suite */
pub const T_DICT_OFS: usize = TEST_OFS+9;
pub const _T_DICT_BEH: Any  = Any { raw: T_DICT_OFS as Raw };
        quad_rom[T_DICT_OFS+0]      = Quad::vm_dict_has(Any::rom(T_DICT_OFS+1));  // #f
        quad_rom[T_DICT_OFS+1]      = Quad::vm_is_eq(FALSE, Any::rom(T_DICT_OFS+2));  // --
        quad_rom[T_DICT_OFS+2]      = Quad::vm_push(NIL, Any::rom(T_DICT_OFS+3));  // ()
        quad_rom[T_DICT_OFS+3]      = Quad::vm_push(ZERO, Any::rom(T_DICT_OFS+4));  // () 0
        quad_rom[T_DICT_OFS+4]      = Quad::vm_dup(PLUS_2, Any::rom(T_DICT_OFS+5));  // () 0 () 0
        quad_rom[T_DICT_OFS+5]      = Quad::vm_dict_has(Any::rom(T_DICT_OFS+6));  // #f
        quad_rom[T_DICT_OFS+6]      = Quad::vm_is_eq(FALSE, Any::rom(T_DICT_OFS+7));  // --
        quad_rom[T_DICT_OFS+7]      = Quad::vm_dict_get(Any::rom(T_DICT_OFS+8));  // #?
        quad_rom[T_DICT_OFS+8]      = Quad::vm_is_eq(UNDEF, Any::rom(T_DICT_OFS+9));  // --

        quad_rom[T_DICT_OFS+9]      = Quad::vm_push(NIL, Any::rom(T_DICT_OFS+10));  // ()
        quad_rom[T_DICT_OFS+10]     = Quad::vm_push(ZERO, Any::rom(T_DICT_OFS+11));  // () 0
        quad_rom[T_DICT_OFS+11]     = Quad::vm_push(UNIT, Any::rom(T_DICT_OFS+12));  // () 0 #unit
        quad_rom[T_DICT_OFS+12]     = Quad::vm_dict_set(Any::rom(T_DICT_OFS+13));  // {0:#unit}
        quad_rom[T_DICT_OFS+13]     = Quad::vm_pick(PLUS_1, Any::rom(T_DICT_OFS+14));  // {0:#unit} {0:#unit}
        quad_rom[T_DICT_OFS+14]     = Quad::vm_push(ZERO, Any::rom(T_DICT_OFS+15));  // {0:#unit} {0:#unit} 0
        quad_rom[T_DICT_OFS+15]     = Quad::vm_dict_get(Any::rom(T_DICT_OFS+16));  // {0:#unit} #unit
        quad_rom[T_DICT_OFS+16]     = Quad::vm_is_eq(UNIT, Any::rom(T_DICT_OFS+17));  // {0:#unit}

        quad_rom[T_DICT_OFS+17]     = Quad::vm_push(PLUS_1, Any::rom(T_DICT_OFS+18));  // {0:#unit} 1
        quad_rom[T_DICT_OFS+18]     = Quad::vm_push(MINUS_1, Any::rom(T_DICT_OFS+19));  // {0:#unit} 1 -1
        quad_rom[T_DICT_OFS+19]     = Quad::vm_dict_add(Any::rom(T_DICT_OFS+20));  // {1:-1, 0:#unit}
        quad_rom[T_DICT_OFS+20]     = Quad::vm_pick(PLUS_1, Any::rom(T_DICT_OFS+21));  // {1:-1, 0:#unit} {1:-1, 0:#unit}
        quad_rom[T_DICT_OFS+21]     = Quad::vm_push(ZERO, Any::rom(T_DICT_OFS+22));  // {1:-1, 0:#unit} {1:-1, 0:#unit} 0
        quad_rom[T_DICT_OFS+22]     = Quad::vm_dict_get(Any::rom(T_DICT_OFS+23));  // {1:-1, 0:#unit} #unit
        quad_rom[T_DICT_OFS+23]     = Quad::vm_is_eq(UNIT, Any::rom(T_DICT_OFS+24));  // {1:-1, 0:#unit}

        quad_rom[T_DICT_OFS+24]     = Quad::vm_push(ZERO, Any::rom(T_DICT_OFS+25));  // {1:-1, 0:#unit} 0
        quad_rom[T_DICT_OFS+25]     = Quad::vm_dict_del(Any::rom(T_DICT_OFS+26));  // {1:-1}
        quad_rom[T_DICT_OFS+26]     = Quad::vm_pick(PLUS_1, Any::rom(T_DICT_OFS+27));  // {1:-1} {1:-1}
        quad_rom[T_DICT_OFS+27]     = Quad::vm_push(ZERO, Any::rom(T_DICT_OFS+28));  // {1:-1} {1:-1} 0
        quad_rom[T_DICT_OFS+28]     = Quad::vm_dict_get(Any::rom(T_DICT_OFS+29));  // {1:-1} #undef
        quad_rom[T_DICT_OFS+29]     = Quad::vm_is_eq(UNDEF, Any::rom(T_DICT_OFS+30));  // {1:-1}

        quad_rom[T_DICT_OFS+30]     = Quad::vm_push(PLUS_1, Any::rom(T_DICT_OFS+31));  // {1:-1} 1
        quad_rom[T_DICT_OFS+31]     = Quad::vm_push(FALSE, Any::rom(T_DICT_OFS+32));  // {1:-1} 1 #f
        quad_rom[T_DICT_OFS+32]     = Quad::vm_dict_add(Any::rom(T_DICT_OFS+33));  // {1:#f, 1:-1}
        quad_rom[T_DICT_OFS+33]     = Quad::vm_pick(PLUS_1, Any::rom(T_DICT_OFS+34));  // {1:#f, 1:-1} {1:#f, 1:-1}
        quad_rom[T_DICT_OFS+34]     = Quad::vm_push(PLUS_1, Any::rom(T_DICT_OFS+35));  // {1:#f, 1:-1} {1:#f, 1:-1} 1
        quad_rom[T_DICT_OFS+35]     = Quad::vm_push(TRUE, Any::rom(T_DICT_OFS+36));  // {1:#f, 1:-1} {1:#f, 1:-1} 1 #t
        quad_rom[T_DICT_OFS+36]     = Quad::vm_dict_set(Any::rom(T_DICT_OFS+37));  // {1:#f, 1:-1} {1:#t, 1:-1}
        quad_rom[T_DICT_OFS+37]     = Quad::vm_pick(PLUS_1, Any::rom(T_DICT_OFS+38));  // {1:#f, 1:-1} {1:#t, 1:-1} {1:#t, 1:-1}
        quad_rom[T_DICT_OFS+38]     = Quad::vm_push(PLUS_1, Any::rom(T_DICT_OFS+39));  // {1:#f, 1:-1} {1:#t, 1:-1} {1:#t, 1:-1} 1
        quad_rom[T_DICT_OFS+39]     = Quad::vm_dict_del(Any::rom(T_DICT_OFS+40));  // {1:#f, 1:-1} {1:#t, 1:-1} {1:-1}

        quad_rom[T_DICT_OFS+40]     = Quad::vm_dup(PLUS_1, Any::rom(T_DICT_OFS+41));  // {1:#f, 1:-1} {1:#t, 1:-1} {1:-1} {1:-1}
        quad_rom[T_DICT_OFS+41]     = Quad::vm_push(PLUS_1, Any::rom(T_DICT_OFS+42));  // {1:#f, 1:-1} {1:#t, 1:-1} {1:-1} {1:-1} 1
        quad_rom[T_DICT_OFS+42]     = Quad::vm_dict_get(Any::rom(T_DICT_OFS+43));  // {1:#f, 1:-1} {1:#t, 1:-1} {1:-1} -1
        quad_rom[T_DICT_OFS+43]     = Quad::vm_is_eq(MINUS_1, COMMIT);  // {1:#f, 1:-1} {1:#t, 1:-1} {1:-1}

        /* VM_DEQUE test suite */
pub const T_DEQUE_OFS: usize = T_DICT_OFS+44;
pub const _T_DEQUE_BEH: Any  = Any { raw: T_DEQUE_OFS as Raw };
        quad_rom[T_DEQUE_OFS+0]     = Quad::vm_deque_empty(Any::rom(T_DEQUE_OFS+1));  // #t
        quad_rom[T_DEQUE_OFS+1]     = Quad::vm_is_eq(TRUE, Any::rom(T_DEQUE_OFS+2));  // --
        quad_rom[T_DEQUE_OFS+2]     = Quad::vm_deque_new(Any::rom(T_DEQUE_OFS+3));  // (())
        quad_rom[T_DEQUE_OFS+3]     = Quad::vm_dup(PLUS_1, Any::rom(T_DEQUE_OFS+4));  // (()) (())
        quad_rom[T_DEQUE_OFS+4]     = Quad::vm_deque_empty(Any::rom(T_DEQUE_OFS+5));  // (()) #t
        quad_rom[T_DEQUE_OFS+5]     = Quad::vm_is_eq(TRUE, Any::rom(T_DEQUE_OFS+6));  // (())

        quad_rom[T_DEQUE_OFS+6]     = Quad::vm_push(PLUS_1, Any::rom(T_DEQUE_OFS+7));  // (()) 1
        quad_rom[T_DEQUE_OFS+7]     = Quad::vm_deque_push(Any::rom(T_DEQUE_OFS+8));  // ((1))
        quad_rom[T_DEQUE_OFS+8]     = Quad::vm_push(PLUS_2, Any::rom(T_DEQUE_OFS+9));  // ((1)) 2
        quad_rom[T_DEQUE_OFS+9]     = Quad::vm_deque_push(Any::rom(T_DEQUE_OFS+10));  // ((2 1))
        quad_rom[T_DEQUE_OFS+10]    = Quad::vm_push(PLUS_3, Any::rom(T_DEQUE_OFS+11));  // ((2 1)) 3
        quad_rom[T_DEQUE_OFS+11]    = Quad::vm_deque_push(Any::rom(T_DEQUE_OFS+12));  // ((3 2 1))
        quad_rom[T_DEQUE_OFS+12]    = Quad::vm_pick(PLUS_1, Any::rom(T_DEQUE_OFS+13));  // ((3 2 1)) ((3 2 1))
        quad_rom[T_DEQUE_OFS+13]    = Quad::vm_deque_empty(Any::rom(T_DEQUE_OFS+14));  // ((3 2 1)) #f
        quad_rom[T_DEQUE_OFS+14]    = Quad::vm_is_eq(FALSE, Any::rom(T_DEQUE_OFS+15));  // ((3 2 1))

        quad_rom[T_DEQUE_OFS+15]    = Quad::vm_pick(PLUS_1, Any::rom(T_DEQUE_OFS+16));  // ((3 2 1)) ((3 2 1))
        quad_rom[T_DEQUE_OFS+16]    = Quad::vm_deque_len(Any::rom(T_DEQUE_OFS+17));  // ((3 2 1)) 3
        quad_rom[T_DEQUE_OFS+17]    = Quad::vm_is_eq(PLUS_3, Any::rom(T_DEQUE_OFS+18));  // ((3 2 1))

        quad_rom[T_DEQUE_OFS+18]    = Quad::vm_deque_pull(Any::rom(T_DEQUE_OFS+19));  // (() 2 3) 1
        quad_rom[T_DEQUE_OFS+19]    = Quad::vm_is_eq(PLUS_1, Any::rom(T_DEQUE_OFS+20));  // (() 2 3)
        quad_rom[T_DEQUE_OFS+20]    = Quad::vm_deque_pull(Any::rom(T_DEQUE_OFS+21));  // (() 3) 2
        quad_rom[T_DEQUE_OFS+21]    = Quad::vm_is_eq(PLUS_2, Any::rom(T_DEQUE_OFS+22));  // (() 3) 2
        quad_rom[T_DEQUE_OFS+22]    = Quad::vm_deque_pull(Any::rom(T_DEQUE_OFS+23));  // (()) 3
        quad_rom[T_DEQUE_OFS+23]    = Quad::vm_is_eq(PLUS_3, Any::rom(T_DEQUE_OFS+24));  // (())
        quad_rom[T_DEQUE_OFS+24]    = Quad::vm_deque_pull(Any::rom(T_DEQUE_OFS+25));  // (()) #?
        quad_rom[T_DEQUE_OFS+25]    = Quad::vm_is_eq(UNDEF, Any::rom(T_DEQUE_OFS+26));  // (())

        quad_rom[T_DEQUE_OFS+26]    = Quad::vm_dup(PLUS_1, Any::rom(T_DEQUE_OFS+27));  // (()) (())
        quad_rom[T_DEQUE_OFS+27]    = Quad::vm_deque_len(Any::rom(T_DEQUE_OFS+28));  // (()) 0
        quad_rom[T_DEQUE_OFS+28]    = Quad::vm_is_eq(ZERO, Any::rom(T_DEQUE_OFS+29));  // (())

        quad_rom[T_DEQUE_OFS+29]    = Quad::vm_dup(PLUS_1, Any::rom(T_DEQUE_OFS+30));  // (()) (())
        quad_rom[T_DEQUE_OFS+30]    = Quad::vm_msg(ZERO, Any::rom(T_DEQUE_OFS+31));  // (()) (()) (@4 #unit)
        quad_rom[T_DEQUE_OFS+31]    = Quad::vm_deque_put(Any::rom(T_DEQUE_OFS+32));  // (()) (() (@4 #unit))
        quad_rom[T_DEQUE_OFS+32]    = Quad::vm_msg(MINUS_1, Any::rom(T_DEQUE_OFS+33));  // (()) (() (@4 #unit)) (#unit)
        quad_rom[T_DEQUE_OFS+33]    = Quad::vm_deque_put(Any::rom(T_DEQUE_OFS+34));  // (()) (() (#unit) (@4 #unit))
        quad_rom[T_DEQUE_OFS+34]    = Quad::vm_msg(MINUS_2, Any::rom(T_DEQUE_OFS+35));  // (()) (() (#unit) (@4 #unit)) ()
        quad_rom[T_DEQUE_OFS+35]    = Quad::vm_deque_put(Any::rom(T_DEQUE_OFS+36));  // (()) (() () (#unit) (@4 #unit))
        quad_rom[T_DEQUE_OFS+36]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+37));  // (()) (((#unit) ())) (@4 #unit)
        quad_rom[T_DEQUE_OFS+37]    = Quad::vm_roll(MINUS_2, Any::rom(T_DEQUE_OFS+38));  // (()) (@4 #unit) (((#unit) ()))
        quad_rom[T_DEQUE_OFS+38]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+39));  // (()) (@4 #unit) ((())) (#unit)
        quad_rom[T_DEQUE_OFS+39]    = Quad::vm_roll(MINUS_3, Any::rom(T_DEQUE_OFS+40));  // (()) (#unit) (@4 #unit) ((()))
        quad_rom[T_DEQUE_OFS+40]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+41));  // (()) (#unit) (@4 #unit) (()) ()
        quad_rom[T_DEQUE_OFS+41]    = Quad::vm_is_eq(NIL, Any::rom(T_DEQUE_OFS+42));  // (()) (#unit) (@4 #unit) (())

        quad_rom[T_DEQUE_OFS+42]    = Quad::vm_push(PLUS_1, Any::rom(T_DEQUE_OFS+43));  // (()) (#unit) (@4 #unit) (()) 1
        quad_rom[T_DEQUE_OFS+43]    = Quad::vm_deque_put(Any::rom(T_DEQUE_OFS+44));  // (()) (#unit) (@4 #unit) (() 1)
        quad_rom[T_DEQUE_OFS+44]    = Quad::vm_push(PLUS_2, Any::rom(T_DEQUE_OFS+45));  // (()) (#unit) (@4 #unit) (() 1) 2
        quad_rom[T_DEQUE_OFS+45]    = Quad::vm_deque_put(Any::rom(T_DEQUE_OFS+46));  // (()) (#unit) (@4 #unit) (() 2 1)
        quad_rom[T_DEQUE_OFS+46]    = Quad::vm_dup(PLUS_1, Any::rom(T_DEQUE_OFS+47));  // (()) (#unit) (@4 #unit) (() 2 1) (() 2 1)
        quad_rom[T_DEQUE_OFS+47]    = Quad::vm_deque_empty(Any::rom(T_DEQUE_OFS+48));  // (()) (#unit) (@4 #unit) (() 2 1) #f
        quad_rom[T_DEQUE_OFS+48]    = Quad::vm_is_eq(FALSE, Any::rom(T_DEQUE_OFS+49));  // (()) (#unit) (@4 #unit) (() 2 1)

        quad_rom[T_DEQUE_OFS+49]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+50));  // (()) (#unit) (@4 #unit) ((2)) 1
        quad_rom[T_DEQUE_OFS+50]    = Quad::vm_is_eq(PLUS_1, Any::rom(T_DEQUE_OFS+51));  // (()) (#unit) (@4 #unit) ((2))
        quad_rom[T_DEQUE_OFS+51]    = Quad::vm_push(PLUS_3, Any::rom(T_DEQUE_OFS+52));  // (()) (#unit) (@4 #unit) ((2)) 3
        quad_rom[T_DEQUE_OFS+52]    = Quad::vm_deque_put(Any::rom(T_DEQUE_OFS+53));  // (()) (#unit) (@4 #unit) ((2) 3)
        quad_rom[T_DEQUE_OFS+53]    = Quad::vm_dup(PLUS_1, Any::rom(T_DEQUE_OFS+54));  // (()) (#unit) (@4 #unit) ((2) 3) ((2) 3)
        quad_rom[T_DEQUE_OFS+54]    = Quad::vm_deque_len(Any::rom(T_DEQUE_OFS+55));  // (()) (#unit) (@4 #unit) ((2) 3) 2
        quad_rom[T_DEQUE_OFS+55]    = Quad::vm_is_eq(PLUS_2, Any::rom(T_DEQUE_OFS+56));  // (()) (#unit) (@4 #unit) ((2) 3)

        quad_rom[T_DEQUE_OFS+56]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+57));  // (()) (#unit) (@4 #unit) (() 3) 2
        quad_rom[T_DEQUE_OFS+57]    = Quad::vm_is_eq(PLUS_2, Any::rom(T_DEQUE_OFS+58));  // (()) (#unit) (@4 #unit) (() 3)
        quad_rom[T_DEQUE_OFS+58]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+59));  // (()) (#unit) (@4 #unit) (()) 3
        quad_rom[T_DEQUE_OFS+59]    = Quad::vm_is_eq(PLUS_3, Any::rom(T_DEQUE_OFS+60));  // (()) (#unit) (@4 #unit) (())
        quad_rom[T_DEQUE_OFS+60]    = Quad::vm_deque_pop(Any::rom(T_DEQUE_OFS+61));  // (()) (#unit) (@4 #unit) (()) #?
        quad_rom[T_DEQUE_OFS+61]    = Quad::vm_is_eq(UNDEF, Any::rom(T_DEQUE_OFS+62));  // (()) (#unit) (@4 #unit) (())
        quad_rom[T_DEQUE_OFS+62]    = Quad::vm_dup(PLUS_1, Any::rom(T_DEQUE_OFS+63));  // (()) (#unit) (@4 #unit) (()) (())
        quad_rom[T_DEQUE_OFS+63]    = Quad::vm_deque_len(Any::rom(T_DEQUE_OFS+64));  // (()) (#unit) (@4 #unit) (()) 0
        quad_rom[T_DEQUE_OFS+64]    = Quad::vm_is_eq(ZERO, COMMIT);  // (()) (#unit) (@4 #unit) (())

        /* device test suite */
pub const T_DEV_OFS: usize = T_DEQUE_OFS+65;
pub const _T_DEV_BEH: Any  = Any { raw: T_DEV_OFS as Raw };
        quad_rom[T_DEV_OFS+0]       = Quad::vm_push(Any::fix(13), Any::rom(T_DEV_OFS+1));  // 13
        quad_rom[T_DEV_OFS+1]       = Quad::vm_push(IO_DEV, Any::rom(T_DEV_OFS+2));  // 13 io_device
        quad_rom[T_DEV_OFS+2]       = Quad::vm_push(BLOB_DEV, Any::rom(T_DEV_OFS+3));  // 13 io_device blob_device
        quad_rom[T_DEV_OFS+3]       = Quad::vm_send(PLUS_2, Any::rom(T_DEV_OFS+4));  // --
        quad_rom[T_DEV_OFS+4]       = Quad::vm_push(Any::fix(3), Any::rom(T_DEV_OFS+5));  // 3
        quad_rom[T_DEV_OFS+5]       = Quad::vm_push(IO_DEV, Any::rom(T_DEV_OFS+6));  // 3 io_device
        quad_rom[T_DEV_OFS+6]       = Quad::vm_push(BLOB_DEV, Any::rom(T_DEV_OFS+7));  // 3 io_device blob_device
        quad_rom[T_DEV_OFS+7]       = Quad::vm_send(PLUS_2, Any::rom(T_DEV_OFS+8));  // --
        quad_rom[T_DEV_OFS+8]       = Quad::vm_drop(ZERO, Any::rom(T_DEV_OFS+12));  // --

        quad_rom[T_DEV_OFS+9]       = Quad::vm_push(IO_DEV, Any::rom(T_DEV_OFS+10));  // io_device
        quad_rom[T_DEV_OFS+10]      = Quad::vm_push(CLOCK_DEV, Any::rom(T_DEV_OFS+11));  // io_device clock_device
        quad_rom[T_DEV_OFS+11]      = Quad::vm_send(ZERO, Any::rom(T_DEV_OFS+12));  // --
        //quad_rom[T_DEV_OFS+11]      = Quad::vm_send(ZERO, COMMIT);  // --

        quad_rom[T_DEV_OFS+12]      = Quad::vm_push(PLUS_5, Any::rom(T_DEV_OFS+13));  // 5
        quad_rom[T_DEV_OFS+13]      = Quad::vm_push(_COUNT_BEH, Any::rom(T_DEV_OFS+14));  // 5 count-beh
        quad_rom[T_DEV_OFS+14]      = Quad::vm_new(ZERO, Any::rom(T_DEV_OFS+15));  // 5 a-count
        quad_rom[T_DEV_OFS+15]      = Quad::vm_send(ZERO, COMMIT);  // --

pub const COUNT_OFS: usize = T_DEV_OFS+16;
pub const _COUNT_BEH: Any  = Any { raw: COUNT_OFS as Raw };
        quad_rom[COUNT_OFS+0]       = Quad::vm_msg(ZERO, Any::rom(COUNT_OFS+1));  // n
        quad_rom[COUNT_OFS+1]       = Quad::vm_dup(PLUS_1, Any::rom(COUNT_OFS+2));  // n n
        quad_rom[COUNT_OFS+2]       = Quad::vm_eq(ZERO, Any::rom(COUNT_OFS+3));  // n n==0
        quad_rom[COUNT_OFS+3]       = Quad::vm_if(ABORT, Any::rom(COUNT_OFS+4));  // n

        quad_rom[COUNT_OFS+4]       = Quad::vm_push(PLUS_1, Any::rom(COUNT_OFS+5));  // n 1
        quad_rom[COUNT_OFS+5]       = Quad::vm_alu_sub(Any::rom(COUNT_OFS+6));  // n-1
        quad_rom[COUNT_OFS+6]       = Quad::vm_my_self(Any::rom(COUNT_OFS+7));  // n-1 self
        quad_rom[COUNT_OFS+7]       = Quad::vm_send(ZERO, COMMIT);  // --

pub const _ROM_TOP_OFS: usize = COUNT_OFS+8;

        /*
         * Random-Access Memory (RAM) image (read/write + GC)
         */
        let mut quad_ram = [
            Quad::empty_t();
            QUAD_RAM_MAX
        ];
        quad_ram[MEMORY.ofs()]      = Quad::memory_t(Any::ram(BNK_INI, _RAM_TOP_OFS), NIL, ZERO, DDEQUE);
        //quad_ram[DDEQUE.ofs()]      = Quad::ddeque_t(NIL, NIL, _K_BOOT, _K_BOOT);
        quad_ram[DDEQUE.ofs()]      = Quad::ddeque_t(NIL, NIL, NIL, NIL);  // no events, no continuations
        quad_ram[BLOB_DEV.ofs()]    = Quad::actor_t(ZERO, NIL, UNDEF);  // blob device #0
        quad_ram[CLOCK_DEV.ofs()]   = Quad::actor_t(PLUS_1, NIL, UNDEF);  // clock device #1
        quad_ram[IO_DEV.ofs()]      = Quad::actor_t(PLUS_2, NIL, UNDEF);  // i/o device #2
        quad_ram[SPONSOR.ofs()]     = Quad::sponsor_t(Any::fix(512), Any::fix(64), Any::fix(512));  // root configuration sponsor
pub const BOOT_OFS: usize = RAM_BASE_OFS;
pub const A_BOOT: Any       = Any { raw: OPQ_RAW | MUT_RAW | BNK_INI | (BOOT_OFS+0) as Raw };
        quad_ram[BOOT_OFS+0]        = Quad::new_actor(SINK_BEH, NIL);
pub const _BOOT_BEH: Any     = Any { raw: MUT_RAW | BNK_INI | (BOOT_OFS+1) as Raw };
        quad_ram[BOOT_OFS+1]        = Quad::vm_push(UNIT, Any::ram(BNK_INI, BOOT_OFS+2));  // #unit
        quad_ram[BOOT_OFS+2]        = Quad::vm_my_self(Any::ram(BNK_INI, BOOT_OFS+3));  // #unit SELF
        //quad_ram[BOOT_OFS+3]        = Quad::vm_push(RESEND, Any::ram(BNK_INI, BOOT_OFS+4));  // #unit SELF resend
        //quad_ram[BOOT_OFS+3]        = Quad::vm_push(_T_DEQUE_BEH, Any::ram(BNK_INI, BOOT_OFS+4));  // #unit SELF test-deque-beh
        quad_ram[BOOT_OFS+3]        = Quad::vm_push(_T_DICT_BEH, Any::ram(BNK_INI, BOOT_OFS+4));  // #unit SELF test-dict-beh
        quad_ram[BOOT_OFS+4]        = Quad::vm_new(ZERO, Any::ram(BNK_INI, BOOT_OFS+5));  // #unit SELF actor
        quad_ram[BOOT_OFS+5]        = Quad::vm_send(PLUS_2, COMMIT);  // --
pub const _BOOT_SP: Any     = Any { raw: MUT_RAW | BNK_INI | (BOOT_OFS+6) as Raw };
        quad_ram[BOOT_OFS+6]        = Quad::pair_t(PLUS_1, Any::ram(BNK_INI, BOOT_OFS+7));
        quad_ram[BOOT_OFS+7]        = Quad::pair_t(PLUS_2, Any::ram(BNK_INI, BOOT_OFS+8));
        quad_ram[BOOT_OFS+8]        = Quad::pair_t(PLUS_3, NIL);
pub const E_BOOT: Any       = Any { raw: MUT_RAW | BNK_INI | (BOOT_OFS+9) as Raw };
        quad_ram[BOOT_OFS+9]        = Quad::new_event(SPONSOR, A_BOOT, NIL);
pub const _K_BOOT: Any       = Any { raw: MUT_RAW | BNK_INI | (BOOT_OFS+10) as Raw };
        //quad_ram[BOOT_OFS+10]       = Quad::new_cont(SINK_BEH, NIL, E_BOOT);
        //quad_ram[BOOT_OFS+10]       = Quad::new_cont(STOP, _BOOT_SP, E_BOOT);
        //quad_ram[BOOT_OFS+10]       = Quad::new_cont(_BOOT_BEH, _BOOT_SP, E_BOOT);
        //quad_ram[BOOT_OFS+10]       = Quad::new_cont(_TEST_BEH, NIL, E_BOOT);
        quad_ram[BOOT_OFS+10]       = Quad::new_cont(_T_DEV_BEH, NIL, E_BOOT);

pub const _RAM_TOP_OFS: usize = BOOT_OFS + 11;

        /*
         * OED-encoded Blob Memory (64kB maximum)
         */
        let mut blob_ram = [
            0x8F as u8;  // fill with OED-encoded `null` octets
            BLOB_RAM_MAX
        ];
        let mut nat = BLOB_RAM_MAX;
        nat -= 9;
        blob_ram[0] = 0x88;             // Array
        blob_ram[1] = 0x82;             //   length: +Integer elements
        blob_ram[2] = 16;               //     length.size = 16 bits
        blob_ram[3] = 1;                //     length[0] = 1 (lsb)
        blob_ram[4] = 0;                //     length[1] = 0 (msb)
        blob_ram[5] = 0x82;             //   size: +Integer octets
        blob_ram[6] = 16;               //     size.size = 16 bits
        blob_ram[7] = u16_lsb(nat);     //     size[0] (lsb)
        blob_ram[8] = u16_msb(nat);     //     size[1] (msb)
        nat -= 9;
        blob_ram[9] = 0x8B;             //   [0] = Extension Blob
        blob_ram[10] = 0x82;            //       meta: +Integer offset
        blob_ram[11] = 16;              //         meta.size = 16 bits
        blob_ram[12] = 0;               //         meta[0] = 0 (lsb)
        blob_ram[13] = 0;               //         meta[1] = 0 (msb)
        blob_ram[14] = 0x82;            //       size: +Integer octets
        blob_ram[15] = 16;              //         size.size = 16 bits
        blob_ram[16] = u16_lsb(nat);    //         size[0] (lsb)
        blob_ram[17] = u16_msb(nat);    //         size[1] (msb)

        Core {
            quad_rom,
            quad_ram0: if BNK_INI == BNK_0 { quad_ram } else { [ Quad::empty_t(); QUAD_RAM_MAX ] },
            quad_ram1: if BNK_INI == BNK_1 { quad_ram } else { [ Quad::empty_t(); QUAD_RAM_MAX ] },
            blob_ram,
            device: [
                Some(Box::new(BlobDevice::new())),
                Some(Box::new(ClockDevice::new())),
                Some(Box::new(IoDevice::new())),
            ],
            rom_top: Any::rom(_ROM_TOP_OFS),
        }
    }

    pub fn run_loop(&mut self) -> bool {  // FIXME: return `Error` instead of `bool`
        loop {
            match self.execute_instruction() {
                Ok(more) => {
                    if !more && !self.event_pending() {
                        return true;  // no more instructions to execute...
                    }
                },
                Err(_error) => {
                    return false;  // limit reached, or error condition signalled...
                },
            }
            if let Err(_error) = self.check_for_interrupt() {
                return false;  // interrupt handler failed...
            }
            if let Err(_error) = self.dispatch_event() {
                return false;  // event dispatch failed...
            }
            // FIXME: if dispatch_event() returns Ok(true), ignore empty k-queue...
        }
    }
    pub fn check_for_interrupt(&mut self) -> Result<bool, Error> {
        //self.gc_stop_the_world();  // FIXME!! REMOVE FORCED GC...
        Ok(false)
        //Err(E_FAIL)
    }
    pub fn dispatch_event(&mut self) -> Result<bool, Error> {
        if !self.event_pending() {
            return Ok(false);  // event queue empty
        }
        let ep = self.e_first();
        let event = self.mem(ep);
        let target = event.x();
        let sponsor = self.event_sponsor(ep);
        let limit = self.sponsor_events(sponsor).fix_num().unwrap_or(0);
        if limit <= 0 {
            return Err(E_MSG_LIM);  // Sponsor event limit reached
        }
        let a_ptr = self.cap_to_ptr(target);
        let a_quad = *self.mem(a_ptr);
        let beh = a_quad.x();
        let state = a_quad.y();
        let events = a_quad.z();
        if let Some(index) = beh.fix_num() {
            // message-event to device
            let id = index as usize;
            if id >= DEVICE_MAX {
                return Err(E_BOUNDS);  // device id must be less than DEVICE_MAX
            }
            let ep_ = self.event_dequeue().unwrap();
            assert_eq!(ep, ep_);
            let mut dev_mut = self.device[id].take().unwrap();
            let result = dev_mut.handle_event(self, ep);
            self.device[id] = Some(dev_mut);
            result  // should normally be Ok(true)
        } else if events == UNDEF {
            // begin actor-event transaction
            let rollback = self.reserve(&a_quad)?;  // snapshot actor state
            let kp = self.new_cont(beh, state, ep)?;  // create continuation
            self.ram_mut(a_ptr).set_z(NIL);  // indicate actor is busy
            self.cont_enqueue(kp);
            let ep_ = self.event_dequeue().unwrap();
            assert_eq!(ep, ep_);
            self.ram_mut(ep).set_z(rollback);  // store rollback in event
            self.set_sponsor_events(sponsor, Any::fix(limit - 1));  // decrement event limit
            Ok(true)  // event dispatched
        } else {
            // target actor is busy, retry later...
            let ep_ = self.event_dequeue().unwrap();
            assert_eq!(ep, ep_);
            self.event_enqueue(ep);  // move event to back of queue
            Ok(false)  // no event dispatched
        }
    }
    pub fn execute_instruction(&mut self) -> Result<bool, Error> {
        let kp = self.kp();
        if !kp.is_ram() {
            return Ok(false);  // continuation queue is empty
        }
        let ep = self.ep();
        let sponsor = self.event_sponsor(ep);
        let limit = self.sponsor_instrs(sponsor).fix_num().unwrap_or(0);
        if limit <= 0 {
            return Err(E_CPU_LIM);  // Sponsor instruction limit reached
        }
        let ip = self.ip();
        let ip_ = self.perform_op(ip)?;
        self.set_ip(ip_);
        self.set_sponsor_instrs(sponsor, Any::fix(limit - 1));
        let kp_ = self.cont_dequeue().unwrap();
        assert_eq!(kp, kp_);
        if self.typeq(INSTR_T, ip_) {
            // re-queue updated continuation
            self.cont_enqueue(kp_);
        } else {
            // free dead continuation and associated event
            self.free(ep);
            self.free(kp);
            self.gc_stop_the_world();  // FIXME!! REMOVE FORCED GC...
        }
        Ok(true)  // instruction executed
    }
    fn perform_op(&mut self, ip: Any) -> Result<Any, Error> {
        let instr = self.mem(ip);
        assert!(instr.t() == INSTR_T);
        let opr = instr.x();  // operation code
        let imm = instr.y();  // immediate argument
        let kip = instr.z();  // next instruction
        let ip_ = match opr {
            VM_TYPEQ => {
                let val = self.stack_pop();
                let r = if self.typeq(imm, val) { TRUE } else { FALSE };
                self.stack_push(r)?;
                kip
            },
            VM_DICT => {
                match imm {
                    DICT_HAS => {
                        let key = self.stack_pop();
                        let dict = self.stack_pop();
                        let b = self.dict_has(dict, key);
                        let v = if b { TRUE } else { FALSE };
                        self.stack_push(v)?;
                    },
                    DICT_GET => {
                        let key = self.stack_pop();
                        let dict = self.stack_pop();
                        let v = self.dict_get(dict, key);
                        self.stack_push(v)?;
                    },
                    DICT_ADD => {
                        let value = self.stack_pop();
                        let key = self.stack_pop();
                        let dict = self.stack_pop();
                        let d = self.dict_add(dict, key, value)?;
                        self.stack_push(d)?;
                    },
                    DICT_SET => {
                        let value = self.stack_pop();
                        let key = self.stack_pop();
                        let dict = self.stack_pop();
                        let d = self.dict_set(dict, key, value)?;
                        self.stack_push(d)?;
                    },
                    DICT_DEL => {
                        let key = self.stack_pop();
                        let dict = self.stack_pop();
                        let d = self.dict_del(dict, key)?;
                        self.stack_push(d)?;
                    },
                    _ => {
                        return Err(E_BOUNDS);  // unknown DICT op
                    }
                };
                kip
            },
            VM_DEQUE => {
                match imm {
                    DEQUE_NEW => {
                        let deque = self.deque_new();
                        self.stack_push(deque)?;
                    },
                    DEQUE_EMPTY => {
                        let deque = self.stack_pop();
                        let b = self.deque_empty(deque);
                        let v = if b { TRUE } else { FALSE };
                        self.stack_push(v)?;
                    },
                    DEQUE_PUSH => {
                        let item = self.stack_pop();
                        let old = self.stack_pop();
                        let new = self.deque_push(old, item)?;
                        self.stack_push(new)?;
                    },
                    DEQUE_POP => {
                        let old = self.stack_pop();
                        let (new, item) = self.deque_pop(old)?;
                        self.stack_push(new)?;
                        self.stack_push(item)?;
                    },
                    DEQUE_PUT => {
                        let item = self.stack_pop();
                        let old = self.stack_pop();
                        let new = self.deque_put(old, item)?;
                        self.stack_push(new)?;
                    },
                    DEQUE_PULL => {
                        let old = self.stack_pop();
                        let (new, item) = self.deque_pull(old)?;
                        self.stack_push(new)?;
                        self.stack_push(item)?;
                    },
                    DEQUE_LEN => {
                        let deque = self.stack_pop();
                        let n = self.deque_len(deque);
                        self.stack_push(Any::fix(n))?;
                    },
                    _ => {
                        return Err(E_BOUNDS);  // unknown DEQUE op
                    }
                };
                kip
            },
            VM_PAIR => {
                let n = imm.get_fix()?;
                self.stack_pairs(n)?;
                kip
            },
            VM_PART => {
                let n = imm.get_fix()?;
                self.stack_parts(n)?;
                kip
            },
            VM_NTH => {
                let lst = self.stack_pop();
                let n = imm.get_fix()?;
                let r = self.extract_nth(lst, n);
                self.stack_push(r)?;
                kip
            },
            VM_PUSH => {
                let val = self.follow_fwd(imm);  // FIXME: may be redundant with low-level memory redirection
                self.stack_push(val)?;
                kip
            },
            VM_DEPTH => {
                let lst = self.sp();
                let n = self.list_len(lst);
                let n = Any::fix(n);
                self.stack_push(n)?;
                kip
            },
            VM_DROP => {
                let mut n = imm.get_fix()?;
                assert!(n < 64);  // FIXME: replace with cycle-limit(s) in Sponsor
                while n > 0 {
                    self.stack_pop();
                    n -= 1;
                };
                kip
            },
            VM_PICK => {
                let n = imm.get_fix()?;
                let r = if n > 0 {
                    let lst = self.sp();
                    self.extract_nth(lst, n)
                } else {
                    UNDEF
                };
                self.stack_push(r)?;
                kip
            },
            VM_DUP => {
                let n = imm.get_fix()?;
                self.stack_dup(n)?;
                kip
            },
            VM_ROLL => {
                let n = imm.get_fix()?;
                self.stack_roll(n)?;
                kip
            },
            VM_ALU => {
                let r = if imm == ALU_NOT {
                    let v = self.stack_pop();
                    match v.fix_num() {
                        Some(n) => Any::fix(!n),
                        _ => UNDEF,
                    }
                } else {
                    let vv = self.stack_pop();
                    let v = self.stack_pop();
                    match (v.fix_num(), vv.fix_num()) {
                        (Some(n), Some(nn)) => {
                            match imm {
                                ALU_AND => Any::fix(n & nn),
                                ALU_OR  => Any::fix(n | nn),
                                ALU_XOR => Any::fix(n ^ nn),
                                ALU_ADD => Any::fix(n + nn),
                                ALU_SUB => Any::fix(n - nn),
                                ALU_MUL => Any::fix(n * nn),
                                _ => UNDEF,
                            }
                        }
                        _ => UNDEF
                    }
                };
                self.stack_push(r)?;
                kip
            },
            VM_EQ => {
                let vv = self.stack_pop();
                let r = if imm == vv { TRUE } else { FALSE };
                self.stack_push(r)?;
                kip
            },
            VM_CMP => {
                let vv = self.stack_pop();
                let v = self.stack_pop();
                let b = if imm == CMP_EQ {
                    v == vv
                } else if imm == CMP_NE {
                    v != vv
                } else {
                    match (v.fix_num(), vv.fix_num()) {
                        (Some(n), Some(nn)) => {
                            match imm {
                                CMP_GE => n >= nn,
                                CMP_GT => n > nn,
                                CMP_LT => n < nn,
                                CMP_LE => n <= nn,
                                _ => false,
                            }
                        }
                        _ => false
                    }
                };
                let r = if b { TRUE } else { FALSE };
                self.stack_push(r)?;
                kip
            },
            VM_IF => {
                let b = self.stack_pop();
                if falsey(b) { kip } else { imm }
            },
            VM_MSG => {
                let n = imm.get_fix()?;
                let ep = self.ep();
                let event = self.mem(ep);
                let msg = event.y();
                let r = self.extract_nth(msg, n);
                self.stack_push(r)?;
                kip
            },
            VM_MY => {
                let me = self.self_ptr();
                match imm {
                    MY_SELF => {
                        let ep = self.ep();
                        let target = self.ram(ep).x();
                        self.stack_push(target)?;
                    },
                    MY_BEH => {
                        let beh = self.ram(me).x();
                        self.stack_push(beh)?;
                    },
                    MY_STATE => {
                        let state = self.ram(me).y();
                        self.push_list(state)?;
                    },
                    _ => {
                        return Err(E_BOUNDS);  // unknown MY op
                    }
                }
                kip
            }
            VM_SEND => {
                let num = imm.get_fix()?;
                let target = self.stack_pop();
                assert!(self.typeq(ACTOR_T, target));
                let msg = if num > 0 {
                    self.pop_counted(num)
                } else {
                    self.stack_pop()
                };
                let ep = self.new_event(target, msg)?;
                let me = self.self_ptr();
                let next = self.ram(me).z();
                if next.is_ram() {
                    self.ram_mut(ep).set_z(next);
                }
                self.ram_mut(me).set_z(ep);
                kip
            },
            VM_NEW => {
                let num = imm.get_fix()?;
                let ip = self.stack_pop();
                assert!(self.typeq(INSTR_T, ip));
                let sp = self.pop_counted(num);
                let a = self.new_actor(ip, sp)?;
                self.stack_push(a)?;
                kip
            },
            VM_BEH => {
                let num = imm.get_fix()?;
                let ip = self.stack_pop();
                assert!(self.typeq(INSTR_T, ip));
                let sp = self.pop_counted(num);
                let me = self.self_ptr();
                let actor = self.ram_mut(me);
                actor.set_x(ip);  // replace behavior function
                actor.set_y(sp);  // replace state data
                kip
            },
            VM_END => {
                let me = self.self_ptr();
                let rv = match imm {
                    END_ABORT => {
                        let _r = self.stack_pop();  // reason for abort
                        // FIXME: where should `reason` be recorded/reported?
                        self.actor_abort(me);
                        UNIT
                    },
                    END_STOP => {
                        //UNDEF
                        return Err(E_FAIL);  // End::Stop terminated continuation
                    },
                    END_COMMIT => {
                        self.actor_commit(me);
                        TRUE
                    },
                    END_RELEASE => {
                        self.ram_mut(me).set_y(NIL);  // no retained stack
                        self.actor_commit(me);
                        self.free(me);  // free actor
                        FALSE
                    },
                    _ => {
                        return Err(E_BOUNDS);  // unknown END op
                    }
                };
                rv
            },
            VM_IS_EQ => {
                let vv = self.stack_pop();
                if imm != vv {
                    return Err(E_ASSERT);  // assertion failed
                }
                kip
            },
            VM_IS_NE => {
                let vv = self.stack_pop();
                if imm == vv {
                    return Err(E_ASSERT);  // assertion failed
                }
                kip
            },
            _ => {
                return Err(E_BOUNDS);  // illegal instruction
            }
        };
        Ok(ip_)
    }

    pub fn event_pending(&self) -> bool {
        self.e_first().is_ram()
    }
    fn event_enqueue(&mut self, ep: Any) {
        // add event to the back of the queue
        self.ram_mut(ep).set_z(NIL);
        if !self.e_first().is_ram() {
            self.set_e_first(ep);
        } else /* if self.e_last().is_ram() */ {
            self.ram_mut(self.e_last()).set_z(ep);
        }
        self.set_e_last(ep);
    }
    fn event_dequeue(&mut self) -> Option<Any> {
        // remove event from the front of the queue
        let ep = self.e_first();
        if ep.is_ram() {
            let event = self.ram(ep);
            let next = event.z();
            self.set_e_first(next);
            if !next.is_ram() {
                self.set_e_last(NIL)
            }
            Some(ep)
        } else {
            None
        }
    }
    pub fn event_inject(&mut self, sponsor: Any, target: Any, msg: Any) -> Result<(), Error> {
        // add event to the front of the queue (e.g.: for interrupts)
        let event = Quad::new_event(sponsor, target, msg);
        let ep = self.reserve(&event)?;  // no Sponsor needed
        let first = self.e_first();
        self.ram_mut(ep).set_z(first);
        if !first.is_ram() {
            self.set_e_last(ep);
        }
        self.set_e_first(ep);
        Ok(())
    }

    fn cont_enqueue(&mut self, kp: Any) {
        // add continuation to the back of the queue
        self.ram_mut(kp).set_z(NIL);
        if !self.k_first().is_ram() {
            self.set_k_first(kp);
        } else /* if self.k_last().is_ram() */ {
            self.ram_mut(self.k_last()).set_z(kp);
        }
        self.set_k_last(kp);
    }
    fn cont_dequeue(&mut self) -> Option<Any> {
        // remove continuation from the front of the queue
        let kp = self.k_first();
        if kp.is_ram() {
            let cont = self.ram(kp);
            let next = cont.z();
            self.set_k_first(next);
            if !next.is_ram() {
                self.set_k_last(NIL)
            }
            Some(kp)
        } else {
            None
        }
    }

    fn actor_commit(&mut self, me: Any) {
        let rollback = self.mem(self.ep()).z();
        if rollback.is_ram() {
            self.free(rollback);  // release rollback snapshot
        }
        let state = self.ram(me).y();
        self.stack_clear(state);
        // move sent-message events to event queue
        let mut ep = self.ram(me).z();
        while ep.is_ram() {
            let event = self.ram(ep);
            let next = event.z();
            self.event_enqueue(ep);
            ep = next;
        }
        // end actor transaction
        self.ram_mut(me).set_z(UNDEF);
    }
    fn actor_abort(&mut self, me: Any) {
        let state = self.ram(me).y();
        self.stack_clear(state);
        // free sent-message events
        let mut ep = self.ram(me).z();
        while ep.is_ram() {
            let event = self.ram(ep);
            let next = event.z();
            self.free(ep);
            ep = next;
        }
        // roll back actor transaction
        ep = self.ep();
        let rollback = self.mem(ep).z();
        if rollback.is_ram() {
            let quad = *self.mem(rollback);
            *self.ram_mut(me) = quad;  // restore actor from rollback
            self.free(rollback);  // release rollback snapshot
        }
    }
    pub fn actor_revert(&mut self) -> bool {
        // revert actor/event to pre-dispatch state
        if let Some(kp) = self.cont_dequeue() {
            let ep = self.mem(kp).y();
            let target = self.mem(ep).x();
            let me = self.cap_to_ptr(target);
            self.actor_abort(me);
            self.event_enqueue(ep);
            true
        } else {
            false
        }
    }
    fn self_ptr(&self) -> Any {
        let ep = self.ep();
        if !ep.is_ram() { return UNDEF }  // no event means no `self`
        let target = self.ram(ep).x();
        let a_ptr = self.cap_to_ptr(target);
        a_ptr
    }

    pub fn sponsor_memory(&self, sponsor: Any) -> Any {
        self.mem(sponsor).t()
    }
    pub fn set_sponsor_memory(&mut self, sponsor: Any, num: Any) {
        self.ram_mut(sponsor).set_t(num);
    }
    pub fn sponsor_events(&self, sponsor: Any) -> Any {
        self.mem(sponsor).x()
    }
    pub fn set_sponsor_events(&mut self, sponsor: Any, num: Any) {
        self.ram_mut(sponsor).set_x(num);
    }
    pub fn sponsor_instrs(&self, sponsor: Any) -> Any {
        self.mem(sponsor).y()
    }
    pub fn set_sponsor_instrs(&mut self, sponsor: Any, num: Any) {
        self.ram_mut(sponsor).set_y(num);
    }
    pub fn event_sponsor(&self, ep: Any) -> Any {
        self.mem(ep).t()
    }

    fn list_len(&self, list: Any) -> isize {
        let mut n: isize = 0;
        let mut p = list;
        while self.typeq(PAIR_T, p) {
            n += 1;
            p = self.cdr(p);
        };
        n
    }
    fn push_list(&mut self, ptr: Any) -> Result<(), Error> {
        if self.typeq(PAIR_T, ptr) {
            self.push_list(self.cdr(ptr))?;
            self.stack_push(self.car(ptr))?;
        }
        Ok(())
    }
    fn pop_counted(&mut self, n: isize) -> Any {
        let mut n = n;
        if n > 0 {  // build list from stack
            let sp = self.sp();
            let mut v = sp;
            let mut p = UNDEF;
            while n > 0 && self.typeq(PAIR_T, v) {
                p = v;
                v = self.cdr(p);
                n -= 1;
            }
            if self.typeq(PAIR_T, p) {
                self.set_cdr(p, NIL);
            }
            self.set_sp(v);
            sp
        } else {  // empty list
            NIL
        }
    }
    fn split_nth(&self, lst: Any, n: isize) -> (Any, Any) {
        // Safely determine the `nth` item of a list and its `pred`ecessor.
        let mut nth = lst;
        let mut pred = UNDEF;
        let mut n = n;
        assert!(n < 64);
        while n > 1 && self.typeq(PAIR_T, nth) {
            pred = nth;
            nth = self.cdr(nth);
            n -= 1;
        }
        (pred, nth)
    }
    fn extract_nth(&self, lst: Any, n: isize) -> Any {
        // Safely extract the `nth` item from a list of Pairs.
        /*
             0          -1          -2          -3
        lst -->[car,cdr]-->[car,cdr]-->[car,cdr]-->...
              +1 |        +2 |        +3 |
                 V           V           V
        */
        let mut p = lst;
        let mut v = UNDEF;
        let mut n = n;
        if n == 0 {  // entire list/message
            v = p;
        } else if n > 0 {  // item at n-th index
            assert!(n < 64);
            while self.typeq(PAIR_T, p) {
                n -= 1;
                if n <= 0 { break; }
                p = self.cdr(p);
            }
            if n == 0 {
                v = self.car(p);
            }
        } else {  // `-n` selects the n-th tail
            assert!(n > -64);
            while self.typeq(PAIR_T, p) {
                n += 1;
                if n >= 0 { break; }
                p = self.cdr(p);
            }
            if n == 0 {
                v = self.cdr(p);
            }
        }
        v
    }

    pub fn dict_has(&self, dict: Any, key: Any) -> bool {
        let mut d = dict;
        while self.typeq(DICT_T, d) {
            let entry = self.mem(d);
            let k = entry.x();  // key
            if key == k {
                return true
            }
            d = entry.z();  // next
        }
        false
    }
    pub fn dict_get(&self, dict: Any, key: Any) -> Any {
        let mut d = dict;
        while self.typeq(DICT_T, d) {
            let entry = self.mem(d);
            let k = entry.x();  // key
            if key == k {
                return entry.y()  // value
            }
            d = entry.z();  // next
        }
        UNDEF
    }
    pub fn dict_add(&mut self, dict: Any, key: Any, value: Any) -> Result<Any, Error> {
        let dict = Quad::dict_t(key, value, dict);
        self.alloc(&dict)
    }
    pub fn dict_set(&mut self, dict: Any, key: Any, value: Any) -> Result<Any, Error> {
        let d = if self.dict_has(dict, key) {
            self.dict_del(dict, key)?
        } else {
            dict
        };
        self.dict_add(d, key, value)
    }
    pub fn dict_del(&mut self, dict: Any, key: Any) -> Result<Any, Error> {
        if self.typeq(DICT_T, dict) {
            let entry = self.mem(dict);
            let k = entry.x();  // key
            let value = entry.y();
            let next = entry.z();
            if key == k {
                Ok(next)
            } else {
                let d = self.dict_del(next, key)?;
                self.dict_add(d, k, value)
            }
        } else {
            Ok(NIL)
        }
    }

    pub fn deque_new(&self) -> Any { EMPTY_DQ }
    pub fn deque_empty(&self, deque: Any) -> bool {
        if self.typeq(PAIR_T, deque) {
            let front = self.car(deque);
            let back = self.cdr(deque);
            !(self.typeq(PAIR_T, front) || self.typeq(PAIR_T, back))
        } else {
            true  // default = empty
        }
    }
    pub fn deque_push(&mut self, deque: Any, item: Any) -> Result<Any, Error> {
        let front = self.car(deque);
        let front = self.cons(item, front)?;
        let back = self.cdr(deque);
        self.cons(front, back)
    }
    pub fn deque_pop(&mut self, deque: Any) -> Result<(Any, Any), Error> {
        if self.typeq(PAIR_T, deque) {
            let mut front = self.car(deque);
            let mut back = self.cdr(deque);
            if !self.typeq(PAIR_T, front) {
                while self.typeq(PAIR_T, back) {
                    // transfer back to front
                    let item = self.car(back);
                    back = self.cdr(back);
                    front = self.cons(item, front)?;
                }
            }
            if self.typeq(PAIR_T, front) {
                let item = self.car(front);
                front = self.cdr(front);
                let deque = self.cons(front, back)?;
                return Ok((deque, item))
            }
        }
        Ok((deque, UNDEF))
    }
    pub fn deque_put(&mut self, deque: Any, item: Any) -> Result<Any, Error> {
        let front = self.car(deque);
        let back = self.cdr(deque);
        let back = self.cons(item, back)?;
        self.cons(front, back)
    }
    pub fn deque_pull(&mut self, deque: Any) -> Result<(Any, Any), Error> {
        if self.typeq(PAIR_T, deque) {
            let mut front = self.car(deque);
            let mut back = self.cdr(deque);
            if !self.typeq(PAIR_T, back) {
                while self.typeq(PAIR_T, front) {
                    // transfer front to back
                    let item = self.car(front);
                    front = self.cdr(front);
                    back = self.cons(item, back)?;
                }
            }
            if self.typeq(PAIR_T, back) {
                let item = self.car(back);
                back = self.cdr(back);
                let deque = self.cons(front, back)?;
                return Ok((deque, item))
            }
        }
        Ok((deque, UNDEF))
    }
    pub fn deque_len(&self, deque: Any) -> isize {
        let front = self.car(deque);
        let back = self.cdr(deque);
        self.list_len(front) + self.list_len(back)
    }

    fn e_first(&self) -> Any { self.ram(self.ddeque()).t() }
    fn set_e_first(&mut self, ptr: Any) { self.ram_mut(self.ddeque()).set_t(ptr); }
    fn e_last(&self) -> Any { self.ram(self.ddeque()).x() }
    fn set_e_last(&mut self, ptr: Any) { self.ram_mut(self.ddeque()).set_x(ptr); }
    fn k_first(&self) -> Any { self.ram(self.ddeque()).y() }
    fn set_k_first(&mut self, ptr: Any) { self.ram_mut(self.ddeque()).set_y(ptr); }
    fn k_last(&self) -> Any { self.ram(self.ddeque()).z() }
    fn set_k_last(&mut self, ptr: Any) { self.ram_mut(self.ddeque()).set_z(ptr); }
    pub fn ddeque(&self) -> Any { self.ptr_to_mem(DDEQUE) }

    pub fn rom_top(&self) -> Any { self.rom_top }
    fn set_rom_top(&mut self, ptr: Any) { self.rom_top = ptr }
    pub fn ram_top(&self) -> Any { self.ram(self.memory()).t() }
    fn set_ram_top(&mut self, ptr: Any) { self.ram_mut(self.memory()).set_t(ptr); }
    fn ram_next(&self) -> Any { self.ram(self.memory()).x() }
    fn set_ram_next(&mut self, ptr: Any) { self.ram_mut(self.memory()).set_x(ptr); }
    fn ram_free(&self) -> Any { self.ram(self.memory()).y() }
    fn set_ram_free(&mut self, fix: Any) { self.ram_mut(self.memory()).set_y(fix); }
    fn ram_root(&self) -> Any { self.ram(self.memory()).z() }
    fn set_ram_root(&mut self, ptr: Any) { self.ram_mut(self.memory()).set_z(ptr); }
    pub fn memory(&self) -> Any { self.ptr_to_mem(MEMORY) }
    pub fn blob_top(&self) -> Any { Any::fix(BLOB_RAM_MAX as isize) }

    fn new_event(&mut self, target: Any, msg: Any) -> Result<Any, Error> {
        assert!(self.typeq(ACTOR_T, target));
        let sponsor = self.event_sponsor(self.ep());
        let event = Quad::new_event(sponsor, target, msg);
        self.alloc(&event)
    }
    fn new_cont(&mut self, ip: Any, sp: Any, ep: Any) -> Result<Any, Error> {
        let cont = Quad::new_cont(ip, sp, ep);
        self.reserve(&cont)  // no Sponsor needed
    }
    fn new_actor(&mut self, beh: Any, state: Any) -> Result<Any, Error> {
        let actor = Quad::new_actor(beh, state);
        let ptr = self.alloc(&actor)?;
        Ok(self.ptr_to_cap(ptr))
    }

    fn stack_pairs(&mut self, n: isize) -> Result<(), Error> {
        assert!(n < 64);  // FIXME: replace with cycle-limit(s) in Sponsor
        if n > 0 {
            let mut n = n;
            let h = self.stack_pop();
            let lst = self.cons(h, NIL)?;
            let mut p = lst;
            while n > 1 {
                let h = self.stack_pop();
                let q = self.cons(h, NIL)?;
                self.set_cdr(p, q);
                p = q;
                n -= 1;
            }
            let t = self.stack_pop();
            self.set_cdr(p, t);
            self.stack_push(lst)?;
        };
        Ok(())
    }
    fn stack_parts(&mut self, n: isize) -> Result<(), Error> {
        assert!(n < 64);  // FIXME: replace with cycle-limit(s) in Sponsor
        let mut s = self.stack_pop();  // list to destructure
        if n > 0 {
            let mut n = n;
            let lst = self.cons(self.car(s), NIL)?;
            let mut p = lst;
            while n > 1 {
                s = self.cdr(s);
                let q = self.cons(self.car(s), NIL)?;
                self.set_cdr(p, q);
                p = q;
                n -= 1;
            }
            let t = self.cons(self.cdr(s), self.sp())?;
            self.set_cdr(p, t);
            self.set_sp(lst);
        }
        Ok(())
    }
    fn stack_roll(&mut self, n: isize) -> Result<(), Error> {
        if n > 1 {
            assert!(n < 64);  // FIXME: replace with cycle-limit(s) in Sponsor
            let sp = self.sp();
            let (pred, nth) = self.split_nth(sp, n);
            if self.typeq(PAIR_T, nth) {
                self.set_cdr(pred, self.cdr(nth));
                self.set_cdr(nth, sp);
                self.set_sp(nth);
            } else {
                self.stack_push(UNDEF)?;  // out of range
            }
        } else if n < -1 {
            assert!(n > -64);  // FIXME: replace with cycle-limit(s) in Sponsor
            let sp = self.sp();
            let (_, nth) = self.split_nth(sp, -n);
            if self.typeq(PAIR_T, nth) {
                self.set_sp(self.cdr(sp));
                self.set_cdr(sp, self.cdr(nth));
                self.set_cdr(nth, sp);
            } else {
                self.stack_pop();  // out of range
            }
        };
        Ok(())
    }
    fn stack_dup(&mut self, n: isize) -> Result<(), Error> {
        let mut n = n;
        if n > 0 {
            let mut s = self.sp();
            let sp = self.cons(self.car(s), NIL)?;
            let mut p = sp;
            s = self.cdr(s);
            n -= 1;
            while n > 0 {
                let q = self.cons(self.car(s), NIL)?;
                self.set_cdr(p, q);
                p = q;
                s = self.cdr(s);
                n -= 1;
            }
            self.set_cdr(p, self.sp());
            self.set_sp(sp);
        }
        Ok(())
    }
    fn stack_clear(&mut self, top: Any) {
        let mut sp = self.sp();
        while sp != top && self.typeq(PAIR_T, sp) {
            let p = sp;
            sp = self.cdr(p);
            self.free(p);  // free pair holding stack item
        }
        self.set_sp(sp);
    }
    fn stack_pop(&mut self) -> Any {
        let sp = self.sp();
        if self.typeq(PAIR_T, sp) {
            let item = self.car(sp);
            self.set_sp(self.cdr(sp));
            // FIXME: avoid inconsistent stack state when hitting memory limits
            //self.free(sp);  // free pair holding stack item
            item
        } else {
            UNDEF  // stack underflow
        }
    }
    fn stack_push(&mut self, val: Any) -> Result<(), Error> {
        let sp = self.cons(val, self.sp())?;
        self.set_sp(sp);
        Ok(())
    }

    pub fn cons(&mut self, car: Any, cdr: Any) -> Result<Any, Error> {
        let pair = Quad::pair_t(car, cdr);
        self.alloc(&pair)
    }
    pub fn nth(&self, list: Any, index: Any) -> Any {
        if let Some(n) = index.fix_num() {
            self.extract_nth(list, n)
        } else {
            UNDEF
        }
    }
    pub fn car(&self, pair: Any) -> Any {
        if self.typeq(PAIR_T, pair) {
            self.mem(pair).x()
        } else {
            UNDEF
        }
    }
    pub fn cdr(&self, pair: Any) -> Any {
        if self.typeq(PAIR_T, pair) {
            self.mem(pair).y()
        } else {
            UNDEF
        }
    }
    fn _set_car(&mut self, pair: Any, val: Any) {
        assert!(self.in_heap(pair));
        assert!(self.ram(pair).t() == PAIR_T);
        self.ram_mut(pair).set_x(val);
    }
    fn set_cdr(&mut self, pair: Any, val: Any) {
        assert!(self.in_heap(pair));
        assert!(self.ram(pair).t() == PAIR_T);
        self.ram_mut(pair).set_y(val);
    }

    pub fn kp(&self) -> Any {  // continuation pointer
        let kp = self.k_first();
        if !kp.is_ram() { return UNDEF }
        kp
    }
    pub fn ip(&self) -> Any {  // instruction pointer
        let kp = self.kp();
        if !kp.is_ram() { return UNDEF }
        let quad = self.mem(kp);
        quad.t()
    }
    pub fn sp(&self) -> Any {  // stack pointer
        let kp = self.kp();
        if !kp.is_ram() { return UNDEF }
        let quad = self.mem(kp);
        quad.x()
    }
    pub fn ep(&self) -> Any {  // event pointer
        let kp = self.kp();
        if !kp.is_ram() { return UNDEF }
        let quad = self.mem(kp);
        quad.y()
    }
    fn set_ip(&mut self, ptr: Any) {
        let quad = self.ram_mut(self.kp());
        quad.set_t(ptr)
    }
    fn set_sp(&mut self, ptr: Any) {
        let quad = self.ram_mut(self.kp());
        quad.set_x(ptr)
    }

    pub fn typeq(&self, typ: Any, val: Any) -> bool {
        if typ == FIXNUM_T {
            val.is_fix()
        } else if typ == ACTOR_T {
            if val.is_cap() {
                // NOTE: we don't use `cap_to_ptr` here to avoid the type assertion.
                let raw = val.raw() & !OPQ_RAW;  // WARNING: converting Cap to Ptr!
                let ptr = Any::new(raw);
                self.mem(ptr).t() == ACTOR_T
            } else {
                false
            }
        } else if val.is_ptr() {
            self.mem(val).t() == typ
        } else {
            false
        }
    }
    pub fn in_heap(&self, val: Any) -> bool {
        val.is_ram() && (val.ofs() < self.ram_top().ofs())
    }
    fn follow_fwd(&self, val: Any) -> Any {
        let raw = val.raw();
        if (raw & (DIR_RAW | MUT_RAW)) == MUT_RAW {  // any RAM reference
            let ptr = Any::new(raw & !OPQ_RAW);  // WARNING: may convert Cap to Ptr!
            let quad = self.ram(ptr);
            if quad.t() == GC_FWD_T {
                let fwd = quad.z();
                return fwd;
            }
        }
        val
    }
    fn ptr_to_mem(&self, ptr: Any) -> Any {  // convert ptr/cap to current gc_phase
        let bank = ptr.bank();
        if bank.is_none() {
            ptr
        } else {
            let raw = ptr.raw() & !BNK_RAW;
            Any::new(self.gc_phase() | raw)
        }
    }
    fn ptr_to_cap(&self, ptr: Any) -> Any {
        assert!(self.mem(ptr).t() == ACTOR_T);
        let raw = ptr.raw() | OPQ_RAW;
        let cap = Any::new(raw);
        cap
    }
    fn cap_to_ptr(&self, cap: Any) -> Any {
        let raw = cap.raw() & !OPQ_RAW;
        let ptr = Any::new(raw);
        assert!(self.mem(ptr).t() == ACTOR_T);
        ptr
    }

    pub fn reserve_rom(&mut self) -> Result<Any, Error> {
        // expand read-only memory
        let next = self.rom_top();
        let top = next.ofs();
        if top >= QUAD_ROM_MAX {
            //panic!("out of memory!");
            return Err(E_NO_MEM);  // no memory available
        }
        self.set_rom_top(Any::rom(top + 1));
        Ok(next)
    }

    pub fn alloc(&mut self, init: &Quad) -> Result<Any, Error> {
        let ep = self.ep();
        let sponsor = self.event_sponsor(ep);
        let limit = self.sponsor_memory(sponsor).fix_num().unwrap_or(0);
        if limit <= 0 {
            //panic!("memory limit reached");
            return Err(E_MEM_LIM);  // Sponsor memory limit reached
        }
        let ptr = self.reserve(init)?;
        self.set_sponsor_memory(sponsor, Any::fix(limit - 1));
        Ok(ptr)
    }
    pub fn reserve(&mut self, init: &Quad) -> Result<Any, Error> {
        let next = self.ram_next();
        let ptr = if self.typeq(FREE_T, next) {
            // use quad from free-list
            let n = self.ram_free().fix_num().unwrap();
            assert!(n > 0);  // number of free cells available
            self.set_ram_free(Any::fix(n - 1));  // decrement cells available
            self.set_ram_next(self.ram(next).z());  // update free-list
            next
        } else {
            // expand top-of-memory
            let next = self.ram_top();
            let top = next.ofs();
            if top >= QUAD_RAM_MAX {
                //panic!("out of memory!");
                return Err(E_NO_MEM);  // no memory available
            }
            self.set_ram_top(Any::ram(self.gc_phase(), top + 1));
            next
        };
        *self.ram_mut(ptr) = *init;  // copy initial value
        Ok(ptr)
    }
    pub fn free(&mut self, ptr: Any) {
        assert!(self.in_heap(ptr));
        if self.typeq(FREE_T, ptr) {
            panic!("double-free {}", ptr.raw());
        }
        *self.ram_mut(ptr) = Quad::free_t(self.ram_next());  // clear cell to "free"
        self.set_ram_next(ptr);  // link into free-list
        let n = self.ram_free().fix_num().unwrap();
        self.set_ram_free(Any::fix(n + 1));  // increment cells available
    }

    pub fn gc_stop_the_world(&mut self) {
        /*
        1. Swap generations (`GC_GENX` <--> `GC_GENY`)
        2. Mark each cell in the root-set with `GC_SCAN`
            1. If a new cell is added to the root-set, mark it with `GC_SCAN`
        3. Mark each newly-allocated cell with `GC_SCAN`
        4. While there are cells marked `GC_SCAN`:
            1. Scan a cell, for each field of the cell:
                1. If it points to the heap, and is marked with the _previous_ generation, mark it `GC_SCAN`
            2. Mark the cell with the _current_ generation
        5. For each cell marked with the _previous_ generation,
            1. Mark the cell `GC_FREE` and add it to the free-cell chain
        */
        let ddeque = self.ddeque();
        let root = self.ram_root();
        let bank = if self.gc_phase() == BNK_0 { BNK_1 } else { BNK_0 };  // determine new phase
        self.set_ram_top(UNDEF);  // toggle GC phase
        self.gc_store(
            Any::ram(bank, MEMORY.ofs()),
            Quad::memory_t(Any::ram(bank, ddeque.ofs()), NIL, ZERO, root));
        let mut scan = ddeque;
        while scan.ofs() < RAM_BASE_OFS {  // mark reserved RAM
            let raw = scan.raw();
            if self.gc_load(scan).t() == ACTOR_T {
                scan = Any::new(raw | OPQ_RAW);  // inferred capability
            }
            self.gc_mark(scan);
            scan = Any::new(raw + 1);
        }
        let root = self.gc_mark(root);
        self.set_ram_root(root);
        scan = self.ddeque();
        while scan != self.ram_top() {  // scan marked quads
            self.gc_scan(scan);
            scan = Any::new(scan.raw() + 1);
        }
    }
    fn gc_mark(&mut self, val: Any) -> Any {
        if let Some(bank) = val.bank() {
            if bank != self.gc_phase() {
                let quad = self.gc_load(val);
                if quad.t() == GC_FWD_T {  // follow "broken heart"
                    return quad.z();
                }
                // copy quad to new-space
                let mut dup = self.reserve(&quad).unwrap();  // FIXME: handle Error result...
                if val.is_cap() {
                    dup = self.ptr_to_cap(dup);  // restore CAP marker
                };
                //println!("gc_mark: {} ${:08x} --> {} ${:08x}", val, val.raw(), dup, dup.raw());
                self.gc_store(val, Quad::gc_fwd_t(dup));  // leave "broken heart" behind
                return dup;
            }
        }
        val
    }
    fn gc_scan(&mut self, ptr: Any) {
        assert_eq!(Some(self.gc_phase()), ptr.bank());
        let quad = self.gc_load(ptr);
        let t = self.gc_mark(quad.t());
        let x = self.gc_mark(quad.x());
        let y = self.gc_mark(quad.y());
        let z = self.gc_mark(quad.z());
        let quad = Quad::new(t, x, y, z);
        self.gc_store(ptr, quad);
    }
    fn gc_load(&self, ptr: Any) -> Quad {  // load quad directly
        match ptr.bank() {
            Some(bank) => {
                let ofs = ptr.ofs();
                if bank == BNK_0 {
                    self.quad_ram0[ofs]
                } else {
                    self.quad_ram1[ofs]
                }
            },
            None => panic!("invalid gc_load=${:08x}", ptr.raw()),
        }
    }
    fn gc_store(&mut self, ptr: Any, quad: Quad) {  // store quad directly
        match ptr.bank() {
            Some(bank) => {
                let ofs = ptr.ofs();
                if bank == BNK_0 {
                    self.quad_ram0[ofs] = quad;
                } else {
                    self.quad_ram1[ofs] = quad;
                }
            },
            None => panic!("invalid gc_store=${:08x}", ptr.raw()),
        }
    }
    pub fn gc_phase(&self) -> Raw {
        let mem = Any::ram(BNK_0, MEMORY.ofs());
        if self.gc_load(mem).t() == UNDEF {
            BNK_1
        } else {
            BNK_0
        }
    }

    pub fn mem(&self, ptr: Any) -> &Quad {
        if !ptr.is_ptr() {
            panic!("invalid ptr=${:08x}", ptr.raw());
        }
        if ptr.is_ram() {
            self.ram(ptr)
        } else {
            self.rom(ptr)
        }
    }
    pub fn rom(&self, ptr: Any) -> &Quad {
        if !ptr.is_rom() {
            panic!("invalid ROM ptr=${:08x}", ptr.raw());
        }
        let ofs = ptr.ofs();
        &self.quad_rom[ofs]
    }
    pub fn ram(&self, ptr: Any) -> &Quad {
        if ptr.is_cap() {
            panic!("opaque ptr=${:08x}", ptr.raw());
        }
        if let Some(bank) = ptr.bank() {
            let ofs = ptr.ofs();
            if bank == BNK_0 {
                &self.quad_ram0[ofs]
            } else {
                &self.quad_ram1[ofs]
            }
        } else {
            panic!("invalid RAM ptr=${:08x}", ptr.raw());
        }
    }
    pub fn ram_mut(&mut self, ptr: Any) -> &mut Quad {
        if ptr.is_cap() {
            panic!("opaque ptr=${:08x}", ptr.raw());
        }
        if let Some(bank) = ptr.bank() {
            let ofs = ptr.ofs();
            if bank == BNK_0 {
                &mut self.quad_ram0[ofs]
            } else {
                &mut self.quad_ram1[ofs]
            }
        } else {
            panic!("invalid RAM ptr=${:08x}", ptr.raw());
        }
    }

    pub fn rom_buffer(&self) -> &[Quad] {
        &self.quad_rom
    }
    pub fn ram_buffer(&self, bank: Raw) -> &[Quad] {
        if bank == BNK_0 {
            &self.quad_ram0
        } else {
            &self.quad_ram1
        }
    }
    pub fn blob_buffer(&self) -> &[u8] {
        &self.blob_ram
    }
    pub fn blob_read(&self, ofs: usize) -> u8 {
        self.blob_ram[ofs]
    }
    pub fn blob_write(&mut self, ofs: usize, data: u8) {
        self.blob_ram[ofs] = data;
    }

    pub fn next(&self, ptr: Any) -> Any {
        if ptr.is_ptr() {
            let quad = self.mem(ptr);
            if quad.t() == INSTR_T {
                let op = quad.x();
                if op == VM_IF || op == VM_END {
                    UNDEF
                } else {
                    quad.z()
                }
            } else if quad.t() == PAIR_T {
                quad.y()
            } else {
                quad.z()
            }
        } else {
            UNDEF
        }
    }
}

fn u16_lsb(nat: usize) -> u8 {
    (nat & 0xFF) as u8
}

fn u16_msb(nat: usize) -> u8 {
    ((nat >> 8) & 0xFF) as u8
}

fn falsey(v: Any) -> bool {
    v == FALSE || v == UNDEF || v == NIL || v == ZERO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_types_are_32_bits() {
        assert_eq!(4, ::core::mem::size_of::<Error>());
        assert_eq!(4, ::core::mem::size_of::<Raw>());
        assert_eq!(4, ::core::mem::size_of::<Num>());
        assert_eq!(4, ::core::mem::size_of::<Any>());
        assert_eq!(16, ::core::mem::size_of::<Quad>());
    }

    #[test]
    fn core_initialization() {
        let core = Core::new();
        assert_eq!(ZERO, core.ram_free());
        assert_eq!(NIL, core.ram_next());
        assert_eq!(NIL, core.e_first());
        assert_eq!(NIL, core.k_first());
        assert_eq!(UNDEF, core.kp());
    }

    #[test]
    fn basic_memory_allocation() {
        let mut core = Core::new();
        let top_before = core.ram_top().ofs();
        let m1 = core.reserve(&Quad::pair_t(PLUS_1, PLUS_1)).unwrap();
        assert!(m1.is_ptr());
        let m2 = core.reserve(&Quad::pair_t(PLUS_2, PLUS_2)).unwrap();
        let m3 = core.reserve(&Quad::pair_t(PLUS_3, PLUS_3)).unwrap();
        core.free(m2);
        core.free(m3);
        let _m4 = core.reserve(&Quad::pair_t(PLUS_4, PLUS_4)).unwrap();
        let top_after = core.ram_top().ofs();
        assert_eq!(3, top_after - top_before);
        assert_eq!(PLUS_1, core.ram_free());
    }

    #[test]
    fn run_loop_terminates() {
        let mut core = Core::new();
        let _ep = core.ep();
        //core.set_sponsor_events(_ep, Any::fix(0));  // FIXME: forcing "out-of-events" error...
        let boot_beh = core.reserve(&Quad::vm_end_commit()).unwrap();
        let boot_ptr = core.reserve(&Quad::new_actor(boot_beh, NIL)).unwrap();
        let a_boot = core.ptr_to_cap(boot_ptr);
        core.event_inject(SPONSOR, a_boot, UNDEF).unwrap();
        let ok = core.run_loop();
        assert!(ok);
    }

    #[test]
    fn gc_before_and_after_run() {
        let mut core = Core::new();
        assert_eq!(BNK_0, core.gc_phase());
        core.gc_stop_the_world();
        assert_eq!(BNK_1, core.gc_phase());
        core.run_loop();
        let bank = core.gc_phase();
        core.gc_stop_the_world();
        assert_ne!(bank, core.gc_phase());
    }

}
