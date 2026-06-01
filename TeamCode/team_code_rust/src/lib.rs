//! Entry point to Rust code.

use ftc::{ftc, hardware::DcMotor};

/// Example linear op mode.
#[ftc(name = "Example: My Linear Op Mode", linear, teleop, group = "Example")]
fn my_linear_op_mode(ctx: &ftc::FtcContext) {
    // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java
    // also fun fact: the syntax `::<T>` where T is a type is affectionately called the turbofish!
    let motor = ctx.hardware().get::<DcMotor>("motor");
    motor.set_direction(ftc::hardware::Direction::Forward);

    ctx.telemetry().add_data("Status", "Initalized");
    ctx.telemetry().update();

    ctx.wait_for_start();

    // ctx.running() instead of opModeIsActive()

    motor.set_power(0.5);
    ctx.sleep_s(2.0);
    motor.set_power(0.0);
}
