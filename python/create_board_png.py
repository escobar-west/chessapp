import png

type Color = tuple[int, int, int]

BOARD_DIM = 8
SQUARE_LENGTH = 80
DARK: Color = (50, 90, 130)
LIGHT: Color = (210, 200, 180)

def create_row(current: Color, next: Color) -> list[int]:
    output = []
    for _ in range(BOARD_DIM):
        output.extend(current * SQUARE_LENGTH)
        current, next = next, current
    return output

def create_board(light: Color, dark: Color) -> list[list[int]]:
    output = []
    current, next = light, dark
    for _ in range(BOARD_DIM):
        for _ in range(SQUARE_LENGTH):
            output.append(create_row(current, next))
        current, next = next, current
    return output


# Create from array
image_2d = create_board(LIGHT, DARK)

# Save as PNG
png.from_array(image_2d, 'RGB').save("output.png")