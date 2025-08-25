import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class RelativeDistance {

    private final Map<String, List<String>> familyTree;

    RelativeDistance(Map<String, List<String>> familyTree) {
        this.familyTree = familyTree;
        System.out.println(familyTree);
    }

    List<String> childrenOf(String person) {
        return this.familyTree.getOrDefault(person, List.of());
    }

    List<String> parentsOf(String person) {
        return this.familyTree.entrySet().stream()
                .filter((entry) -> entry.getValue().contains(person))
                .map(Map.Entry::getKey).toList();
    }

    List<String> siblingsOf(String person) {
        var parents = this.parentsOf(person);
        return this.familyTree.entrySet().stream()
                .filter((entry) -> parents.contains(entry.getKey()))
                .flatMap((entry) -> entry.getValue().stream()).toList();
    }

    Set<String> familyOf(String person) {
        return Stream.of(
                        parentsOf(person).stream(),
                        childrenOf(person).stream(),
                        siblingsOf(person).stream()
                )
                .flatMap(p -> p)
                .collect(Collectors.toSet());
    }

    int degreeOfSeparation(String personA, String personB) {
        var visited = new HashSet<String>();
        var visitQueue = new LinkedList<Map.Entry<String, Stack<String>>>();
        var startEntry = Map.entry(personA, new Stack<String>());
        startEntry.getValue().push(personA);
        visitQueue.push(startEntry);

        while (!visitQueue.isEmpty()) {
            var currentEntry = visitQueue.pollFirst();
            var someone = currentEntry.getKey();
            var path = currentEntry.getValue();

            if (visited.contains(someone)) continue;
            visited.add(someone);

            var someFamily = this.familyOf(someone);
            if (someFamily.contains(personB)) {
                System.out.println(path);
                return path.size();
            }
            someFamily.stream()
                    .filter(person -> !visited.contains(person))
                    .forEach((person) -> {
                        var nextPath = new Stack<String>();
                        nextPath.addAll(path);
                        nextPath.push(person);
                        var next = Map.entry(person, nextPath);
                        visitQueue.add(next);
                    });
        }
        return -1;
    }
}