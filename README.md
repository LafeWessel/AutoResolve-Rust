# AutoResolve-Rust
My AutoResolve program implemented in Rust.

Performs same functionality as AutoResolve2.0

## Resource Files

### equipment.csv

Order of variables in `equipment.csv` file:
1. Type: A(Armor), W(Weapon), T(Trinket), B(Banner), F(Follower) (string/char)
2. Name (string)
3. Effect(s) (string)
4. Coin Value (int)
5. Index Value (int)
6. Autoresolve Bonus (int)
7. Range (int)

The 'Corpse Thief' Follower equipment has 1 as it's autoresolve bonus to help calculate bonuses when finding treasure at the end of battles.

### units.csv

Order of Variables in `units.csv` file:
1. Faction: 1,2,3, or 4, follows the enum class defined below (int)
2. Name (string)
3. Type: Melee(1), Cavalry(2) or Ranged(3) (int)
4. Autoresolve Bonus (int)
5. Unit size(int)

## Data Capture

Data can be saved from test runs to `.csv` format.

### Format

Columns in output from 4 regular battle types (Normal, Siege, Raid, Naval):
Scroll down for read out of locations for Monster battle type

0. Battle Type - string
1. Battle Randoms Attacker Total - int
2. Battle Randoms Defender Total - int
3. Ending Total - double
4. Outcome - string
5. Attacker Won - bool(int)
6. Supplies - int

Attacker:
7. General Rank - int
8. General Total Autoresolve Bonus - int
9. General Armor Bonus - int
10. General Weapon Bonus - int
11. General Follower Bonus - int
12. General Banner Bonus - int
13. General Trinket Bonus - int
14. Advanced Combat Deck - int
15. Total Beginning Autoresolve Unit Bonus - int
16. Total Beginning Melee Bonus - int
17. Total Beginning Ranged Bonus - int
18. Total Beginning Cavalry Bonus - int
19. Total Beginning Soldiers - int
20. Total Ending Autoresolve Unit Bonus - int
21. Total Ending Melee Bonus - int
22. Total Ending Ranged Bonus - int
23. Total Ending Cavalry Bonus - int
24. Total Ending Soldiers - int
25. Amount of Rebel Rivermen Mercenaries
26. Amount of Rebel Rangers of the Forest
27. Amount of Rebel Courier Riders
28. Amount of Rebel Mountain men
29. Amount of Rebel Axemen of the Vale
30. Amount of Rebel Vale Cavalry
31. Amount of Rebel Light Woodmen
32. Amount of Rebel Woodsmen Archers
33. Amount of Rebel Riders of the Wood
34. Amount of Light Woodmen
35. Amount of Guards of Beladimir
36. Amount of Beladimir Spearmen
37. Amount of Woodsmen Archers
38. Amount of Archers of Beladimir
39. Amount of Coastal Guards
40. Amount of Riders of the Wood
41. Amount of Beladimir Cavalry
42. Amount of Plains Cavalry
43. Amount of Mountain Men
44. Amount of Lerastir Shieldbearers
45. Amount of Lerastir Spears
46. Amount of Pikes of Lerastir
47. Amount of Axemen of the Vale
48. Amount of Bowmen of Lerastir
49. Amount of Crossbowmen of the South
50. Amount of Vale Cavalry
51. Amount of Territory Guardsmen
52. Lerastir Palace Cavalry
53. Amount of Shieldmaidens of the North
54. Amount of Menoriad Spearmen
55. Amout of Axemen of the Wastes
56. Amount of Menoriad Archers
57. Amount of Rangers of the Forests
58. Amount of Courier Riders
59. Amount of Menoriad Cavalry
60. Amount of Royal Cavalry
61. Amount of Hunters of the North
62. Amount of Rivermen Mercenaries
63. Total Units - int
64. Total Reinforcements - int
65. Total Upgrades Received - int
66. Total Unit Casualties - int
67. Total Soldier Casualties - int
68. End General State - string
69. Treasure Received - bool(int)
70. Faction - string
71. Ships - int
72. Rams - int
73. Siege Towers - int
74. Catapults - int

Defender:
75. General Rank - int
76. General Total Autoresolve Bonus - int
77. General Armor Bonus - int
78. General Weapon Bonus - int
79. General Follower Bonus - int
80. General Banner Bonus - int
81. General Trinket Bonus - int
82. Advanced Combat Deck - int
83. Total Beginning Autoresolve Unit Bonus - int
84. Total Beginning Melee Bonus - int
85. Total Beginning Ranged Bonus - int
86. Total Beginning Cavalry Bonus - int
87. Total Beginning Soldiers - int
88. Total Ending Autoresolve Unit Bonus - int
89. Total Ending Melee Bonus - int
90. Total Ending Ranged Bonus - int
91. Total Ending Cavalry Bonus - int
92. Total Ending Soldiers - int
93. Amount of Rebel Rivermen Mercenaries
94. Amount of Rebel Rangers of the Forest
95. Amount of Rebel Courier Riders
96. Amount of Rebel Mountain men
97. Amount of Rebel Axemen of the Vale
98. Amount of Rebel Vale Cavalry
99. Amount of Rebel Light Woodmen
100. Amount of Rebel Woodsmen Archers
101. Amount of Rebel Riders of the Wood
102. Amount of Light Woodmen
103. Amount of Guards of Beladimir
104. Amount of Beladimir Spearmen
105. Amount of Woodsmen Archers
106. Amount of Archers of Beladimir
107. Amount of Coastal Guards
108. Amount of Riders of the Wood
109. Amount of Beladimir Cavalry
110. Amount of Plains Cavalry
111. Amount of Mountain Men
112. Amount of Lerastir Shieldbearers
113. Amount of Lerastir Spears
114. Amount of Pikes of Lerastir
115. Amount of Axemen of the Vale
116. Amount of Bowmen of Lerastir
117. Amount of Crossbowmen of the South
118. Amount of Vale Cavalry
119. Amount of Territory Guardsmen
120. Lerastir Palace Cavalry
121. Amount of Shieldmaidens of the North
122. Amount of Menoriad Spearmen
123. Amout of Axemen of the Wastes
124. Amount of Menoriad Archers
125. Amount of Rangers of the Forests
126. Amount of Courier Riders
127. Amount of Menoriad Cavalry
128. Amount of Royal Cavalry
129. Amount of Hunters of the North
130. Amount of Rivermen Mercenaries
131. Total Units - int
132. Total Reinforcements - int
133. Total Upgrades Received - int
134. Total Unit Casualties - int
135. Total Soldier Casualties - int
136. End General State - int
137. Treasure Received - bool(int)
138. Faction - string
139. Ships - int
140. TownStat Level - int


Monster Battle:
All are same as above unless listed otherwise

75.	Monster type - string
76.	Monster Coin Reward
77.	Monster Total AR Value
