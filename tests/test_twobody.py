import matplotlib.pyplot as plt
from twobody_py import simulate_moon_earth_py

result = simulate_moon_earth_py(1.0,5000000)
body1_x = result[0]
body1_y = result[1]
body2_x = result[2]
body2_y = result[3]

plt.figure(figsize=(8,6))
plt.scatter(body1_x, body1_y, s=1, c='blue', label='Earth')
plt.scatter(body2_x, body2_y, s=1, c='gray', label='Moon')
plt.title('Earth-Moon System')
plt.xlabel('x')
plt.ylabel('y')
plt.legend()
plt.grid(True)
plt.show()