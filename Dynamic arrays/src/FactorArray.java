/**
 * @author Wamdue
 * @version 1.0
 * @since 12.05.2019
 */
public class FactorArray<T> implements IArrays<T> {

    private int size;
    private Object[] array;
    private int factor;

    public FactorArray(int len, int factor) {
        this.factor = factor;
        array = new Object[len];
        size = 0;
    }

    public FactorArray() {
        this(10, 50);
    }


    @Override
    public int size() {
        return size;
    }

    @Override
    public void add(T item) {
        resize(size);
        array[size] = item;
        size++;
    }

    @Override
    public void add(T item, int index) {
        resize(index);
        array[index] = item;
    }

    @Override
    @SuppressWarnings("unchecked")
    public T remove(int index) {
        T temp = (T) array[index];
        System.arraycopy(array, index + 1, array, index, size - 1);
        size--;
        return temp;
    }

    @Override
    @SuppressWarnings("unchecked")
    public T get(int index) {
        return (T) array[index];
    }

    private void resize(int index) {
        if (size == array.length || index > size) {
            Object[] tempArray = newArray();
            System.arraycopy(array, 0, tempArray, 0, index);
            System.arraycopy(array, index, tempArray, index + 1, size - index);
            array = tempArray;
        } else {
            System.arraycopy(array, 0, array, 0, index);
            System.arraycopy(array, index, array, index + 1, size - index);
        }
    }

    private Object[] newArray() {
        return new Object[size + size * factor / 100];
    }
}
