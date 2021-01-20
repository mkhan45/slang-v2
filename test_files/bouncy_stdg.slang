# run with https://github.com/calebwin/stdg/releases/tag/v0.2.0

let screen_x = 600
let screen_y = 600

let x = 200.0
let y = 200.0

let x_vel = 0.3
let y_vel = 0.2

let radius = 20.0

print("start " + screen_x + " " + screen_y + " Bouncy")

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

while(true) {
    print(concat(["background", 0, 0, 0]))
    print(concat(["fill", 255, 255, 255, 255]))
    print(concat(["stroke", 255, 255, 255]))
    print(concat(["circle", x, y, radius]))
    print("present")

    x += x_vel
    y += y_vel

    if ((x - radius < 0) || (x + radius > screen_x)) {
        x_vel = x_vel * -1.0
    }

    if ((y - radius < 0) || (y + radius > screen_y)) {
        y_vel = y_vel * -1.0
    }
}
