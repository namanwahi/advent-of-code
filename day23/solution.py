from collections import deque    

if __name__ == "__main__":
    cups = [2, 1, 5, 6, 9, 4, 7, 8, 3]
    cups = cups + list(range(max(cups) + 1, 1_000_001))

    min_label = min(cups)
    max_label = max(cups)

    # build mapping from current to next
    circle = {}
    for i, cup in enumerate(cups):
        circle[cup] = cups[(i + 1) % len(cups)]

    current_label = cups[0]
    for move in range(10_000_000):

        # pick up next three cups
        picked_up = [
            circle[current_label],
            circle[circle[current_label]],
            circle[circle[circle[current_label]]],
        ]

        # close off the picked up cups from the circle
        circle[current_label] = circle[circle[circle[circle[current_label]]]]

        # find dest label
        destination_label = current_label
        while destination_label in picked_up + [current_label]:
            destination_label -= 1
            if destination_label < min_label:
                destination_label = max_label

        # add picked up back to the circle
        old_next_after_dest = circle[destination_label]
        circle[destination_label] = picked_up[0]
        circle[picked_up[-1]] = old_next_after_dest

        current_label = circle[current_label]

    curr = circle[1]
    print("Part 2", circle[1] * circle[circle[1]])