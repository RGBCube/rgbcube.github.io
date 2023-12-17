from PIL import Image, ImageDraw

width, height = 1024, 1024

canvas = Image.new("RGB", (width, height))
draw   = ImageDraw.Draw(canvas)

sides = {
    "front": [
        (000, 255, 255),
        (255, 255, 255),
        (000, 000, 255),
        (255, 000, 255),
    ],
    "top": [
        (000, 255, 000),
        (255, 255, 000),
        (000, 255, 255),
        (255, 255, 255),
    ],
    "back": [
        (255, 255, 000),
        (000, 255, 000),
        (255, 000, 000),
        (000, 000, 000),
    ],
    "bottom": [
        (000, 000, 255),
        (255, 000, 255),
        (000, 000, 000),
        (255, 000, 000),
    ],
    "right": [
        (255, 255, 255),
        (255, 255, 000),
        (255, 000, 255),
        (255, 000, 000),
    ],
    "left": [
        (000, 255, 000),
        (000, 255, 255),
        (000, 000, 000),
        (000, 000, 255),
    ],
}

for side, colors in sides.items():
    top_left, top_right = colors[0], colors[1]
    bottom_left, bottom_right = colors[2], colors[3]

    for y in range(height):
        left = (
            int((bottom_left[0] - top_left[0]) * y / height) + top_left[0],
            int((bottom_left[1] - top_left[1]) * y / height) + top_left[1],
            int((bottom_left[2] - top_left[2]) * y / height) + top_left[2]
        )
        right = (
            int((bottom_right[0] - top_right[0]) * y / height) + top_right[0],
            int((bottom_right[1] - top_right[1]) * y / height) + top_right[1],
            int((bottom_right[2] - top_right[2]) * y / height) + top_right[2]
        )

        for x in range(width):
            r = int((right[0] - left[0]) * x / width) + left[0]
            g = int((right[1] - left[1]) * x / width) + left[1]
            b = int((right[2] - left[2]) * x / width) + left[2]

            draw.point((x, y), fill=(r, g, b))

    canvas.save(f"{side}.png")
