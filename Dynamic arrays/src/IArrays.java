public interface IArrays<T> {

    int size();

    void add(T item);

    void add (T item, int index);

    T remove(int index);

    T get(int index);
}
