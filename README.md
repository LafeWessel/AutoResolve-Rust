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

Columns in Read out from 4 regular battle types:
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
25. -62 Amount of Each Unit(38 types) - int
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
93. -130	Amount of Each Unit(38 types) - int
131.	Total Units - int
132.	Total Reinforcements - int
133.	Total Upgrades Received - int
134.	Total Unit Casualties - int
135.	Total Soldier Casualties - int
136.	End General State - int
137.	Treasure Received - bool(int)
138.	Faction - string

139.	Ships - int
140.	TownStat Level - int


Monster Battle:
All are same as above unless listed otherwise

75.	Monster type - string
76.	Monster Coin Reward
77.	Monster Total AR Value
