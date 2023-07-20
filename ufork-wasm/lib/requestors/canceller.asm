; The "canceller" actor can be passed as the 'to_cancel' capability in a request
; message. You can pretty much treat it like the cancel capability that the
; requestor may eventually send.

; It is rare to provide a meaningful reason for cancellation, but if it is then
; the reason must be wrapped in a list:

;   (reason) -> canceller

; A more cancel-like capability can be derived from a canceller by wrapping it
; in 'wrap_beh' from lib.asm.

.import
    std: "../std.asm"
    dev: "../dev.asm"
    lib: "../lib.asm"

beh:
canceller_beh:                  ; () <- message
    msg 0                       ; message
    typeq #actor_t              ; cap?
    if_not got_reason           ; --
    msg 0                       ; cancel
    push wait_for_reason_beh    ; cancel wait_for_reason_beh
    beh -1                      ; --
    ref std.commit

got_reason:
    msg 0                       ; (reason)
    push wait_for_cancel_beh    ; (reason) wait_for_cancel_beh
    beh -1                      ; --
    ref std.commit

wait_for_cancel_beh:            ; (reason) <- message
    msg 0                       ; message
    typeq #actor_t              ; cap?
    if_not std.commit           ; --
    state 1                     ; reason
    msg 0                       ; reason cancel
    ref send_reason_to_cancel

wait_for_reason_beh:            ; cancel <- message
    msg 0                       ; message
    typeq #actor_t              ; cap?
    if std.commit               ; --
    msg 1                       ; reason
    state 0                     ; reason cancel
send_reason_to_cancel:          ; reason cancel
    push std.sink_beh           ; reason cancel sink_beh
    beh 0                       ; reason cancel
    ref std.send_msg

; Test suite

boot:                   ; () <- {caps}

; Cancel arrives before reason.

    msg 0               ; {caps}
    push 50             ; {caps} cancel_ms
    push 100            ; {caps} cancel_ms reason_ms
    push 42             ; {caps} cancel_ms reason_ms reason
    push test_beh       ; {caps} cancel_ms reason_ms reason test_beh
    new 3               ; {caps} test=test_beh.(reason reason_ms cancel_ms)
    send -1             ; --

; Reason arrives before cancel.

    msg 0               ; {caps}
    push 100            ; {caps} cancel_ms
    push 50             ; {caps} cancel_ms reason_ms
    push 1729           ; {caps} cancel_ms reason_ms reason
    push test_beh       ; {caps} cancel_ms reason_ms reason test_beh
    new 3               ; {caps} test=test_beh.(reason reason_ms cancel_ms)
    send -1             ; --
    ref std.commit

; We create a canceller and send it a cancel capability and a reason, each after
; a different delay. Each is sent twice, to test the canceller's tolerance.

test_beh:               ; (reason reason_ms cancel_ms) <- {caps}
    push canceller_beh  ; canceller_beh
    new 0               ; canceller=canceller_beh.()
    state 0             ; canceller (reason ...)
    pick 2              ; canceller (reason ...) canceller
    state 2             ; canceller (reason ...) canceller reason_ms
    msg 0               ; canceller (reason ...) canceller reason_ms {caps}
    push dev.timer_key  ; canceller (reason ...) canceller reason_ms {caps} timer_key
    dict get            ; canceller (reason ...) canceller reason_ms timer_dev
    dup 4               ; ... (reason ...) canceller reason_ms timer_dev
    send 3              ; ... (reason ...) canceller reason_ms timer_dev
    send 3              ; canceller
    msg 0               ; canceller {caps}
    push dev.debug_key  ; canceller {caps} debug_key
    dict get            ; canceller debug_dev
    pick 2              ; canceller debug_dev canceller
    state 3             ; canceller debug_dev canceller cancel_ms
    msg 0               ; canceller debug_dev canceller cancel_ms {caps}
    push dev.timer_key  ; canceller debug_dev canceller cancel_ms {caps} timer_key
    dict get            ; canceller debug_dev canceller cancel_ms timer_dev
    dup 4               ; ... debug_dev canceller cancel_ms timer_dev
    send 3              ; ... debug_dev canceller cancel_ms timer_dev
    send 3              ; canceller
    ref std.commit

.export
    beh
    boot
