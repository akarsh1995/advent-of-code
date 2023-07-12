# 208. Implement Trie (Prefix Tree)
# Medium

# A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.

# Implement the Trie class:

#     Trie() Initializes the trie object.
#     void insert(String word) Inserts the string word into the trie.
#     boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
#     boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.

#

# Example 1:

# Input
# ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
# [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
# Output
# [null, null, true, false, true, null, true]

# Explanation
# Trie trie = new Trie();
# trie.insert("apple");
# trie.search("apple");   // return True
# trie.search("app");     // return False
# trie.startsWith("app"); // return True
# trie.insert("app");
# trie.search("app");     // return True

from collections import defaultdict

class TreeNode:
    def __init__(self):
        self.is_end = False
        self.next = defaultdict(TreeNode)


class Trie:

    def __init__(self):
        self.root = TreeNode()

    def insert(self, word: str) -> None:
        curr = self.root
        for l in word:
            if not l in curr.next:
                curr.next[l]
            curr = curr.next[l]
        curr.is_end = True

    def search(self, word: str) -> bool:
        curr = self.root
        for l in word:
            if not l in curr.next:
                return False
            curr = curr.next[l]
        return curr.is_end


    def startsWith(self, prefix: str) -> bool:
        curr = self.root
        for l in prefix:
            if not l in curr.next:
                return False
            curr = curr.next[l]
        return True


def test():
    a = zip( ["Trie", "insert", "search", "search", "startsWith", "insert", "search"], [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]], [None, None, True, False, True, None, True])

    t = Trie()
    for instruction, word, result in a:
        if instruction == "Trie":
            continue
        elif instruction == "insert":
            t.insert(word[0])
        elif instruction == "search":
            assert t.search(word[0]) == result
        else:
            assert t.startsWith(word[0]) == result


'''
common mistake:
    Forgets to create a separate class for the TreeObject, mistake to try to fit in the node iteself
'''
