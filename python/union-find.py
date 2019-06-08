class UnionFind(object):
    def __init__(self, nodes):
        self.__parents = {node: None for node in nodes}

    def union(self, a, b):
        root_of_a = self.find(a)
        root_of_b = self.find(b)
        if root_of_a != root_of_b:
            self.__parents[root_of_a] = root_of_b

    def find(self, a):
        if self.__parents[a] is None:
            return a
        root = self.find(self.__parents[a])
        self.__parents[a] = root
        return root

    @property
    def parents(self):
        return {key: value for key, value in self.__parents.items()}
