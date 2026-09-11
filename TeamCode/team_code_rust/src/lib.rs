//! Example Rust opmodes. You can have as many op modes as you want in each file.
use std::time::Duration;

use ftc::{PressEdge, ftc, hardware::DcMotor, log::info};

/// Example linear op mode.
#[ftc(name = "Example: My Linear Op Mode", linear, teleop, group = "Example")]
fn my_linear_op_mode(ftc: &ftc::FtcContext) {
    // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java
    // also fun fact: the syntax `::<T>` where T is a type is affectionately called the turbofish!
    let motor = ftc.hardware().get::<DcMotor>("motor");
    motor.set_direction(ftc::hardware::Direction::Forward);

    ftc.telemetry().add_data("Status", "Initialized");
    ftc.telemetry().update();

    ftc.wait_for_start();

    // you can store it here, or just call the method directly in the body; storing it once is
    // slightly more efficient
    let gamepad1 = ftc.gamepad1();

    while ftc.running() {
        let power = f64::from(gamepad1.left_stick_y());
        ftc.telemetry().add_data("Status", "Running");
        ftc.telemetry().add_data("Power", power);
        ftc.telemetry().update();

        motor.set_power(power);
    }
}

/// State used in the iterative op mode. Essentially equivalent to adding properties to a class in
/// java. Has to implement Default (which can be derived in most scenarios as you see below) and
/// some other requirements the compiler will enforce.
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
    iterative.init(|ftc: &ftc::FtcContext, state: &mut IterativeState| {
        // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java:
        state.motor = ftc.hardware().get::<DcMotor>("motor");
        state.motor.set_direction(ftc::hardware::Direction::Forward);

        let gamepad1 = ftc.gamepad1();

        gamepad1.on_a(
            move |ftc, _| {
                ftc.telemetry().add_data("A", "Pressed");
            },
            PressEdge::Press,
        );
        gamepad1.on_a(
            move |ftc, _| {
                ftc.telemetry().add_data("A", "Released");
            },
            PressEdge::Release,
        );

        ftc.telemetry().add_data("Status", "Initialized");
        ftc.telemetry().update();
    });

    iterative.start(|_ftc, state: &mut IterativeState| {
        state.motor.set_power(0.5);
        std::thread::sleep(Duration::from_secs_f32(2.0));
        state.motor.set_power(0.0);
    });

    iterative.stop(|ftc, _state: &mut IterativeState| {
        // state has to have a type, so use the state type and ignore the value.
        info!("Ran for {:?}!", ftc.runtime());
    });

    // attempting to call wait_for_start in a interative op mode will immediately return and
    // print a warning
}
