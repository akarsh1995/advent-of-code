# 49. Group Anagrams
# Given an array of strings strs, group the anagrams together. You can return the answer in any order.

# Example 1:

# Input: strs = ["eat","tea","tan","ate","nat","bat"]
# Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

from collections import Counter, defaultdict
from typing import List

def get_word_key(word):
    if word == "":
        return ""
    l = [0 for _ in range(26)]
    for char in word:
        l[ord(char) - ord('a')] += 1
    return ",".join(map(str, l))

def get_words(strs: List[str]):
    word_sort = defaultdict(list)
    for w in strs:
        word_sort[get_word_key(w)].append(w)
    return list(word_sort.values())


def test():
    assert get_words(["eat","tea","tan","ate","nat","bat"]) == [['eat', 'tea', 'ate'], ['tan', 'nat'], ['bat']]

    
