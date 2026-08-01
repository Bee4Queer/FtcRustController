//! Example Rust opmodes.
use std::time::Duration;

use ftc::{
    ftc,
    hardware::{CRServo, DcMotor},
    log::info,
};

/// Example linear op mode.
#[ftc(name = "Example: My Linear Op Mode", linear, teleop, group = "Example")]
fn my_linear_op_mode(ctx: &ftc::FtcContext) {
    // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java
    // also fun fact: the syntax `::<T>` where T is a type is affectionately called the turbofish!
    let motor = ctx.hardware().get::<DcMotor>("motor");
    let servo = ctx.hardware().get::<CRServo>("servo");
    motor.set_direction(ftc::hardware::Direction::Forward);

    ctx.telemetry().add_data("Status", "Initalized");
    ctx.telemetry().update();

    ctx.wait_for_start();

    // ctx.running() instead of opModeIsActive()

    servo.set_power(1.0);
    motor.set_power(0.5);
    std::thread::sleep(Duration::from_secs_f32(2.0));
    motor.set_power(0.0);
    servo.set_power(0.0);
    std::thread::sleep(Duration::from_secs_f32(0.5));
}

/// State used in the iterative op mode. Essentially equivalent to adding properties to a class in
/// java. Has to implement Java and not have any non-static references, as well as being Send + Sync
/// (Send means safe to move across threads, Sync means safe to move references to the type across
/// threads).
#[derive(Default)]
struct IterativeState {
    /// Devices implement Default by returning a null object of sorts that panics
    /// if you use it, but comes in handy for stuff like this.
    motor: DcMotor,
}

/// Example iterative op mode.
#[ftc(
    name = "Example: My Iterative Op Mode",
    iterative,
    teleop,
    group = "Example"
)]
fn my_iterative_op_mode(iterative: &ftc::IterativeContext) {
    iterative.init(|ctx: &ftc::FtcContext, state: &mut IterativeState| {
        // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java:
        state.motor = ctx.hardware().get::<DcMotor>("motor");
        state.motor.set_direction(ftc::hardware::Direction::Forward);

        ctx.telemetry().add_data("Status", "Initalized");
        ctx.telemetry().update();
    });

    iterative.start(|_ctx, state: &mut IterativeState| {
        state.motor.set_power(0.5);
        std::thread::sleep(Duration::from_secs_f32(2.0));
        state.motor.set_power(0.0);
    });

    iterative.stop(|ctx, _state: &mut ()| {
        // state has to have a type, so use the unit type and ignore the value
        info!("Ran for {:?}!", ctx.runtime());
    });

    // attempting to call wait_for_start with an interative context will immediately return and
    // print a warning
}
