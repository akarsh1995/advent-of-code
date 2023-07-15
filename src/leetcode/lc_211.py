# 211. Design Add and Search Words Data Structure
# Medium

# Design a data structure that supports adding new words and finding if a string matches any previously added string.

# Implement the WordDictionary class:

#     WordDictionary() Initializes the object.
#     void addWord(word) Adds word to the data structure, it can be matched later.
#     bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.

#

# Example:

# Input
# ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
# [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
# Output
# [null,null,null,null,false,true,true,true]

# Explanation
# WordDictionary wordDictionary = new WordDictionary();
# wordDictionary.addWord("bad");
# wordDictionary.addWord("dad");
# wordDictionary.addWord("mad");
# wordDictionary.search("pad"); // return False
# wordDictionary.search("bad"); // return True
# wordDictionary.search(".ad"); // return True
# wordDictionary.search("b.."); // return True


class TrieNode:
    def __init__(self):
        self.map = {}
        self.is_terminal = False


class WordDictionary:

    def __init__(self):
        self.start_node = TrieNode()

    def addWord(self, word: str) -> None:
        curr = self.start_node
        for l in word:
            if not l in curr.map:
                curr.map[l] = TrieNode()
            curr = curr.map[l]
        curr.is_terminal = True

    def helper(self, word: str, curr: TrieNode) -> bool:
        for i, l in enumerate(word):
            if l in curr.map:
                curr = curr.map[l]
            else:
                if l == ".":
                    return any(self.helper(word[i + 1:], v) for _, v in curr.map.items())
                else:
                    return False
        return curr.is_terminal


    def search(self, word: str) -> bool:
        curr = self.start_node
        return self.helper(word, curr)


def test():
    op = ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
    input = [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
    w = WordDictionary()
    comp = [None,None,None,None,False,True,True,True]

    for i, (o, patt) in enumerate(zip(op, input)):
        if i == 0:
            continue
        if o.startswith("add"):
            assert w.addWord(patt[0]) == comp[i]
        else:
            assert w.search(patt[0]) == comp[i]
