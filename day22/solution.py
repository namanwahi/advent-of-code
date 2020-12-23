P1_START = [
#    9,
#    2,
#    6,
#    3,
#    1,
    2,
    31,
    14,
    45,
    33,
    18,
    29,
    36,
    44,
    47,
    38,
    6,
    9,
    5,
    48,
    17,
    50,
    41,
    4,
    21,
    42,
    23,
    25,
    28,
    3,
]

P2_START = [
#    5,
#    8,
#    4,
#    7,
#    10,
    26,
    16,
    27,
    12,
    49,
    32,
    19,
    46,
    37,
    15,
    10,
    30,
    11,
    24,
    1,
    40,
    7,
    8,
    43,
    34,
    20,
    35,
    22,
    39,
    13,
]

def recursive_combat(player_1, player_2):
    seen_states = set()
    player_1 = player_1.copy()
    player_2 = player_2.copy()

    while True:
        game_state = tuple(player_1 + [-1] + player_2)
        if game_state in seen_states:
            return 1, sum([(len(player_1) - i) * val for (i, val) in enumerate(player_1)])
        seen_states.add(game_state)

        p1_play = player_1.pop(0)
        p2_play = player_2.pop(0)

        if len(player_1) >= p1_play and len(player_2) >= p2_play:
            round_winner, _ = recursive_combat(player_1[:p1_play], player_2[:p2_play])
        else:
            round_winner = 1 if p1_play > p2_play else 2

        if round_winner == 1:
            player_1.append(p1_play)
            player_1.append(p2_play)
        elif round_winner == 2:
            player_2.append(p2_play)
            player_2.append(p1_play)        
        else:
            assert False

        if not player_1:
            return 2, sum([(len(player_2) - i) * val for (i, val) in enumerate(player_2)])
        
        if not player_2:
            return 1, sum([(len(player_1) - i) * val for (i, val) in enumerate(player_1)])

if __name__ == "__main__":

    # PART 1
    player_1 = P1_START[:]
    player_2 = P2_START[:]
    end = False
    while not end:
        p1_play = player_1[0]
        p2_play = player_2[0]
        assert p1_play != p2_play

        if p1_play > p2_play:
            player_1.append(player_1.pop(0))
            player_1.append(player_2.pop(0))
        else:
            player_2.append(player_2.pop(0))
            player_2.append(player_1.pop(0))

        if not player_1 or not player_2:
            end = True
    
    winning_deck = player_1 if player_1 else player_2

    winning_score = sum([(len(winning_deck) - i) * val for (i, val) in enumerate(winning_deck)])

    print("Part 1", winning_score)

    # PART 2
    print("Part 2", recursive_combat(P1_START, P2_START)[-1])
        
