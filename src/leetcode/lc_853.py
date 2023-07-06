# 853. Car Fleet
# Medium

# There are n cars going to the same destination along a one-lane road. The destination is target miles away.
# You are given two integer array position and speed, both of length n, where position[i] is the position of the ith car and speed[i] is the speed of the ith car (in miles per hour).
# A car can never pass another car ahead of it, but it can catch up to it and drive bumper to bumper at the same speed. The faster car will slow down to match the slower car's speed. The distance between these two cars is ignored (i.e., they are assumed to have the same position).
# A car fleet is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.
# If a car catches up to a car fleet right at the destination point, it will still be considered as one car fleet.
# Return the number of car fleets that will arrive at the destination.

# Example 1:

# Input: target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
# Output: 3
# Explanation:
# The cars starting at 10 (speed 2) and 8 (speed 4) become a fleet, meeting each other at 12.
# The car starting at 0 does not catch up to any other car, so it is a fleet by itself.
# The cars starting at 5 (speed 1) and 3 (speed 3) become a fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.
# Note that no other cars meet these fleets before the destination, so the answer is 3.

from typing import List

def get_fleet_count(position: List[int], speed: List[int], target_pos: int):
    distance_time_array = [(p, (target_pos - p) / s)  for p, s in zip(position, speed)]

    # sort cars by positions
    distance_time_array.sort(key=lambda x, : x[0])

    # reverse distance_time_array
    distance_time_array = distance_time_array[::-1]
    # appending large number to avoid list ending checks
    distance_time_array.append((4954, 940385094))

    i = 0
    j = 1

    fleets = 0

    while j < len(distance_time_array) and i < j:
        car_behind_reaches_earlier = distance_time_array[i][1] > distance_time_array[j][1]
        car_behind_reaches_later_than_car_ahead = distance_time_array[i][1] < distance_time_array[j][1]
        car_behind_reached_same_time_car_ahead = distance_time_array[i][1] == distance_time_array[j][1]

        if car_behind_reaches_earlier:
            j += 1
        elif car_behind_reaches_later_than_car_ahead:
            fleets += 1
            i = j
            j += 1
        elif car_behind_reached_same_time_car_ahead:
            i = j
            j = i + 1
    return fleets

def test():
    position = [10,8,0,5,3]
    speed = [2,4,1,1,3]
    target = 12
    assert get_fleet_count(position, speed, target) == 3

def test2():
    position = [3]
    speed = [3]
    target = 10
    assert get_fleet_count(position, speed, target) == 1

def test3():
    position = [0, 2, 4]
    speed = [4, 2, 1]
    target = 100
    assert get_fleet_count(position, speed, target) == 1
