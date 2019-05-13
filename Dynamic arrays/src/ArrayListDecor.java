import java.util.ArrayList;

public class ArrayListDecor<T> implements IArrays<T> {
    private ArrayList<T> arrayList;

    ArrayListDecor() {
        arrayList = new ArrayList<>();
    }

    @Override
    public int size() {
        return arrayList.size();
    }

    @Override
    public void add(T item) {
        arrayList.add(item);
    }

    @Override
    public void add(T item, int index) {
        arrayList.add(index, item);
    }

    @Override
    public T remove(int index) {
        return arrayList.remove(index);
    }

    @Override
    public T get(int index) {
        return arrayList.get(index);
    }
}
