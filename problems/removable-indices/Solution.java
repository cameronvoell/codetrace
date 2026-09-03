import java.io.*;
import java.util.*;
import static java.util.stream.Collectors.joining;

class Result {

    /*
     * str1 is exactly one character longer than str2.
     * Return every index of str1 whose removal makes str1 equal to str2,
     * in ascending order. If there is none, return a list containing -1.
     */
    public static List<Integer> getRemovableIndices(String str1, String str2) {
        // TODO
        return List.of(-1);
    }
}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        String str1 = br.readLine();
        String str2 = br.readLine();
        List<Integer> result = Result.getRemovableIndices(str1, str2);
        System.out.println(result.stream().map(Object::toString).collect(joining("\n")));
        br.close();
    }
}
