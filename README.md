# AutoResolve-Rust
My AutoResolve program implemented in Rust.

Performs same functionality as AutoResolve2.0

## CLI Flags

### `-b`/`--battle`

Specify which type of battle to run. 
- 1: Normal
- 2: Siege
- 3: Raid
- 4: Naval
- 5: Monster

### `-c`/`--count`

Number of battle calculations to perform.

### `-h`/`--help`

Display help information

### `-j`/`--json`

JSON file to read and parse to battle. Mutually exclusive with `r`/`--random` and `-b`/`--battle`.

### `-l`/`--log`

Display results from each battle run

### `-r`/`--random`

Use randomly generated data for battle runs. Mutually exclusive with `-j`/`--json`.

### `-s`/`--save`

Save battle results to `.csv` file. Can override default save location with `-f`/`--file`.

### `--treasure`

Override default file for reading in treasure data

### `--unit`

Override default file for reading in unit data

## Resource Files

### equipment.csv

Order of variables in `equipment.csv` file:
1. Type: Armor, Weapon, Trinket, Banner, Follower (`String`)
2. Name (`String`)
3. Effect(s) (`String`)
4. Coin Value (`int`)
5. Id (`unsigned int`)
6. Autoresolve Bonus (`int`)
7. Range (`int`)

The 'Corpse Thief' Follower equipment has 1 as it's autoresolve bonus to help calculate bonuses when finding treasure at the end of battles.

### units.csv

Order of Variables in `units.csv` file:
1. Faction: Rebel, Beladimir, Lerastir, Menoriad (`String`)
2. Name (`String`)
3. Type: Melee, Cavalry, or Ranged (`String`)
4. Autoresolve Bonus (`int`)
5. Unit size(`int`)
6. Unit id(`unsigned int`)

## Data Capture

Data can be saved from test runs to `.csv` format.

### Format

Columns in output from 4 regular battle types (Normal, Siege, Raid, Naval):
Scroll down for read out of locations for Monster battle type

0. Battle Type - `String`
1. Battle Randoms Attacker Total - `int`
2. Battle Randoms Defender Total - `int`
3. Ending Total - `float`
4. Outcome - `String`
5. Attacker Won - bool(`int`)
6. Supplies - `int`

Attacker:

7. General Rank - `int`
8. General Total Autoresolve Bonus - `int`
9. General Armor Bonus - `int`
10. General Weapon Bonus - `int`
11. General Follower Bonus - `int`
12. General Banner Bonus - `int`
13. General Trinket Bonus - `int`
14. Advanced Combat Deck - `int`
15. Total Beginning Autoresolve Unit Bonus - `int`
16. Total Beginning Melee Bonus - `int`
17. Total Beginning Ranged Bonus - `int`
18. Total Beginning Cavalry Bonus - `int`
19. Total Beginning Soldiers - `int`
20. Total Ending Autoresolve Unit Bonus - `int`
21. Total Ending Melee Bonus - `int`
22. Total Ending Ranged Bonus - `int`
23. Total Ending Cavalry Bonus - `int`
24. Total Ending Soldiers - `int`
25. Amount of Rebel Rivermen Mercenaries - `int`
26. Amount of Rebel Rangers of the Forest - `int`
27. Amount of Rebel Courier Riders - `int`
28. Amount of Rebel Mountain men - `int`
29. Amount of Rebel Axemen of the Vale - `int`
30. Amount of Rebel Vale Cavalry - `int`
31. Amount of Rebel Light Woodmen - `int`
32. Amount of Rebel Woodsmen Archers - `int`
33. Amount of Rebel Riders of the Wood - `int`
34. Amount of Light Woodmen - `int`
35. Amount of Guards of Beladimir - `int`
36. Amount of Beladimir Spearmen - `int`
37. Amount of Woodsmen Archers - `int`
38. Amount of Archers of Beladimir - `int`
39. Amount of Coastal Guards - `int`
40. Amount of Riders of the Wood - `int`
41. Amount of Beladimir Cavalry - `int`
42. Amount of Plains Cavalry - `int`
43. Amount of Mountain Men - `int`
44. Amount of Lerastir Shieldbearers - `int`
45. Amount of Lerastir Spears - `int`
46. Amount of Pikes of Lerastir - `int`
47. Amount of Axemen of the Vale - `int`
48. Amount of Bowmen of Lerastir - `int`
49. Amount of Crossbowmen of the South - `int`
50. Amount of Vale Cavalry - `int`
51. Amount of Territory Guardsmen - `int`
52. Lerastir Palace Cavalry - `int`
53. Amount of Shieldmaidens of the North - `int`
54. Amount of Menoriad Spearmen - `int`
55. Amout of Axemen of the Wastes - `int`
56. Amount of Menoriad Archers - `int`
57. Amount of Rangers of the Forests - `int`
58. Amount of Courier Riders - `int`
59. Amount of Menoriad Cavalry - `int`
60. Amount of Royal Cavalry - `int`
61. Amount of Hunters of the North - `int`
62. Amount of Rivermen Mercenaries - `int`
63. Total Units - `int`
64. Total Reinforcements - `int`
65. Total Upgrades Received - `int`
66. Total Unit Casualties - `int`
67. Total Soldier Casualties - `int`
68. End General State - `String`
69. Treasure Received - bool(`int`)
70. Faction - string
71. Ships - `int`
72. Rams - `int`
73. Siege Towers - `int`
74. Catapults - `int`

Defender:

75. General Rank - `int`
76. General Total Autoresolve Bonus - `int`
77. General Armor Bonus - `int`
78. General Weapon Bonus - `int`
79. General Follower Bonus - `int`
80. General Banner Bonus - `int`
81. General Trinket Bonus - `int`
82. Advanced Combat Deck - `int`
83. Total Beginning Autoresolve Unit Bonus - `int`
84. Total Beginning Melee Bonus - `int`
85. Total Beginning Ranged Bonus - `int`
86. Total Beginning Cavalry Bonus - `int`
87. Total Beginning Soldiers - `int`
88. Total Ending Autoresolve Unit Bonus - `int`
89. Total Ending Melee Bonus - `int`
90. Total Ending Ranged Bonus - `int`
91. Total Ending Cavalry Bonus - `int`
92. Total Ending Soldiers - `int`
93. Amount of Rebel Rivermen Mercenaries - `int`
94. Amount of Rebel Rangers of the Forest - `int`
95. Amount of Rebel Courier Riders - `int`
96. Amount of Rebel Mountain men - `int`
97. Amount of Rebel Axemen of the Vale - `int`
98. Amount of Rebel Vale Cavalry - `int`
99. Amount of Rebel Light Woodmen - `int`
100. Amount of Rebel Woodsmen Archers - `int`
101. Amount of Rebel Riders of the Wood - `int`
102. Amount of Light Woodmen - `int`
103. Amount of Guards of Beladimir - `int`
104. Amount of Beladimir Spearmen - `int`
105. Amount of Woodsmen Archers - `int`
106. Amount of Archers of Beladimir - `int`
107. Amount of Coastal Guards - `int`
108. Amount of Riders of the Wood - `int`
109. Amount of Beladimir Cavalry - `int`
110. Amount of Plains Cavalry - `int`
111. Amount of Mountain Men - `int`
112. Amount of Lerastir Shieldbearers - `int`
113. Amount of Lerastir Spears - `int`
114. Amount of Pikes of Lerastir - `int`
115. Amount of Axemen of the Vale - `int`
116. Amount of Bowmen of Lerastir - `int`
117. Amount of Crossbowmen of the South - `int`
118. Amount of Vale Cavalry - `int`
119. Amount of Territory Guardsmen - `int`
120. Lerastir Palace Cavalry - `int`
121. Amount of Shieldmaidens of the North - `int`
122. Amount of Menoriad Spearmen - `int`
123. Amout of Axemen of the Wastes - `int`
124. Amount of Menoriad Archers - `int`
125. Amount of Rangers of the Forests - `int`
126. Amount of Courier Riders - `int`
127. Amount of Menoriad Cavalry - `int`
128. Amount of Royal Cavalry - `int`
129. Amount of Hunters of the North - `int`
130. Amount of Rivermen Mercenaries - `int`
131. Total Units - `int`
132. Total Reinforcements - `int`
133. Total Upgrades Received - `int`
134. Total Unit Casualties - `int`
135. Total Soldier Casualties - `int`
136. End General State - `int`
137. Treasure Received - bool(`int`)
138. Faction - `String`
139. Ships - `int`
140. TownStat Level - `int`


Monster Battle:
All are same as above unless listed otherwise

75.	Monster type - `String`
76.	Monster Coin Reward - `int`
77.	Monster Total AR Value - `int`
