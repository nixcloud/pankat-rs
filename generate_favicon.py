from PIL import Image, ImageDraw
import io
import sys

try:
    # Create a new image with a 32x32 pixel size (standard favicon size)
    img = Image.new('RGB', (32, 32), color='#2b2b2b')
    draw = ImageDraw.Draw(img)

    # Draw a filled circle as background
    draw.ellipse((2, 2, 30, 30), fill='#4169E1')  # Royal Blue

    # Draw 'R' in white, positioned in center
    # Since we can't rely on specific fonts, we'll draw a simple geometric 'R'
    # Vertical line
    draw.rectangle((8, 6, 12, 26), fill='white')
    # Top curve
    draw.arc((8, 6, 20, 16), 270, 90, fill='white', width=4)
    # Diagonal line
    draw.line((12, 16, 24, 26), fill='white', width=4)

    # Save as ICO
    img.save('frontend/static/favicon.ico', format='ICO')
    print("Favicon generated successfully!")
except Exception as e:
    print(f"Error generating favicon: {e}", file=sys.stderr)
    sys.exit(1)