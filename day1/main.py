from pyfiglet import Figlet
import pyfiglet
f = Figlet()

text1 = f.renderText("Dobry wieczor")
text2 = f.renderText("Jak mija dzień ?")

print(text1)
print(text2)

# Step 2
text3 = pyfiglet.figlet_format("CodeXmas", font="banner")
print(text3)

text_hello = pyfiglet.figlet_format("Hello World", font="larry3d")
print(text_hello)

text_rounded = pyfiglet.figlet_format("Square", font="rounded")
print(text_rounded)

text_classic = pyfiglet.figlet_format("Compaq Desktop", font="roman")
print(text_classic)

text_classic2 = pyfiglet.figlet_format("AMD Micro Computers", font="rev")
print(text_classic2)

# Step 3
f2 = Figlet(font='tinker-toy')
text_tinkey = f2.renderText("Tinkey")
text_tinkey2 = f2.renderText("Toy Story")

print(text_tinkey)
print(text_tinkey2)