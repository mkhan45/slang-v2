# run with https://github.com/calebwin/stdg/releases/tag/v0.2.0

let screen_x = 600.0
let screen_y = 600.0

let ball_x = screen_x / 2.0
let ball_y = screen_y / 2.0
let ball_x_vel = 0.3
let ball_y_vel = 0.6
let ball_radius = 15.0

let paddle_thickness = 10.0
let paddle_length = 85.0
let paddle_offset = 20.0
let paddle_speed = 0.8

let top_paddle_x = screen_x / 2.0
let top_paddle_y = paddle_offset
let bottom_paddle_x = screen_x / 2.0
let bottom_paddle_y = screen_y - paddle_offset

fn concat(arr) {
    let sum = ""

    for (let i = 0; i < len(arr); i += 1) {
        sum = sum + arr[i]
        if (i != len(arr) - 1) {
            sum = sum + " "
        }
    }

    sum
}

fn travel(x, target_x, speed) {
    let diff = target_x - x

    let new_x = x

    if (diff > speed) {
        new_x = x + speed
    }

    if (diff < -speed) {
        new_x = x - speed
    }

    new_x
}

# calculates the x position of the ball at the target_y position
fn calculate_ball_position(ball_x, ball_y, x_vel, y_vel, target_y) {
    # basically just pretend the bounces don't exist
    # and then mod by the screen width to wrap it around

    let y_dist = target_y - ball_y

    let time_to_y_pos = y_dist / y_vel

    if (time_to_y_pos < 0.0) {
        time_to_y_pos = time_to_y_pos * -2.0
    }
    
    let total_x_travel = x_vel * time_to_y_pos

    let initial_x_offset = ball_x
    if (x_vel < 0.0) {
        initial_x_offset = screen_x - ball_x
    }

    let absolute_x_pos = round(total_x_travel + initial_x_offset)

    let mod_fac = round(screen_x)
    let res = (absolute_x_pos % mod_fac) + 0.0
    let num_bounces = floor(absolute_x_pos / (mod_fac))

    if (res < 0) {
         res = res * -1.0 + paddle_length / 2.0
    }
    if (num_bounces % 2 == 1) {
        res = screen_x - res + 0.0
    }

    res
}

print(concat(["start", screen_x, screen_y, "Bouncy"]))

let bottom_target_x = calculate_ball_position(ball_x, ball_y, ball_x_vel, ball_y_vel, bottom_paddle_y - paddle_thickness / 2.0)
let top_target_x = calculate_ball_position(ball_x, ball_y, ball_x_vel, ball_y_vel, top_paddle_y + paddle_thickness / 2.0)

for (;;) {
    print(concat(["background", 0, 0, 0]))
    print(concat(["fill", 255, 255, 255]))
    print(concat(["stroke", 255, 255, 255]))

    print(concat(["circle", ball_x, ball_y, ball_radius]))

    print(concat(["rect", bottom_paddle_x, screen_y - paddle_offset, paddle_length, paddle_thickness]))

    print(concat(["rect", top_paddle_x, paddle_offset, paddle_length, paddle_thickness]))

    print("present")

    ball_x += ball_x_vel
    ball_y += ball_y_vel


    if ((ball_x - ball_radius < 0) || (ball_x + ball_radius > screen_x)) {
        ball_x_vel = ball_x_vel * -1.0
    }

    if ((ball_y + ball_radius) > bottom_paddle_y - paddle_thickness / 2.0) {
        # ball is on y level to collide with bottom paddle
        let paddle_l_bound = bottom_paddle_x
        let paddle_r_bound = bottom_paddle_x + paddle_length
        if ((ball_x > paddle_l_bound) && (ball_x < paddle_r_bound) && (ball_y_vel > 0.0)) {
            ball_y_vel = ball_y_vel * -1.0

            let target_x = calculate_ball_position(ball_x, ball_y, ball_x_vel, ball_y_vel, top_paddle_y + paddle_thickness / 2.0)
            if (target_x > screen_x - paddle_length / 2.0) {
                target_x = screen_x - paddle_length / 2.0
            }
            if (target_x < 0) {
                target_x = 0.0
            }

            top_target_x = target_x - paddle_length / 2.0
        }
    }

    if ((ball_y - ball_radius) < top_paddle_y + paddle_thickness / 2.0) {
        # ball is on y level to collide with top paddle
        let paddle_l_bound = top_paddle_x
        let paddle_r_bound = top_paddle_x + paddle_length
        if ((ball_x > paddle_l_bound) && (ball_x < paddle_r_bound) && (ball_y_vel < 0)) {
            ball_y_vel = ball_y_vel * -1.0

            let target_x = calculate_ball_position(ball_x, ball_y, ball_x_vel, ball_y_vel, bottom_paddle_y - paddle_thickness / 2.0)
            if (target_x > screen_x - paddle_length / 2.0) {
                target_x = screen_x - paddle_length / 2.0
            }
            if (target_x < 0.0) {
                target_x = 0.0
            }

            bottom_target_x = target_x - paddle_length / 2.0
        }
    }

    let top_new_x = travel(top_paddle_x, top_target_x, paddle_speed)
    top_paddle_x = top_new_x

    let bottom_new_x = travel(bottom_paddle_x, bottom_target_x, paddle_speed)
    bottom_paddle_x = bottom_new_x
}
