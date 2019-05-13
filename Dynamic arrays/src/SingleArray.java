public class SingleArray<T> implements IArrays<T> {

    Object[] array;

    public SingleArray() {
        array = new Object[0];
    }

    @Override
    public int size() {
        return array.length;
    }

    @Override
    public void add(T item) {
        resize();
        array[size() - 1] = item;
    }

    @Override
    public void add(T item, int index) {
        Object[] temp = new Object[size() + 1];
        System.arraycopy(array, 0, temp,0, index);
        System.arraycopy(array, index, temp, index + 1, size() - index);
        temp[index] = item;
        array = temp;
    }

    @Override
    @SuppressWarnings("unchecked")
    public T remove(int index) {
        Object[] tempArray = new Object[size() - 1];
        T temp = (T) array[index];
        System.arraycopy(array, 0, tempArray, 0, index);
        System.arraycopy(array, index + 1, tempArray, index, size() - index - 1);
        array = tempArray;
        return temp;
    }

    @Override
    @SuppressWarnings("unchecked")
    public T get(int index) {
        return (T) array[index];
    }

    private void resize() {
        Object[] newArray = new Object[size() + 1];
        System.arraycopy(array, 0, newArray, 0, size());
        array = newArray;
    }
}
