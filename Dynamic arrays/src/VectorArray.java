public class VectorArray<T>implements IArrays<T> {

    private int size;
    private Object[] array;
    private int vector;

    public VectorArray(int vector) {
        this.vector = vector;
        array = new Object[vector];
        size = 0;
    }
    public VectorArray() {
        this(10);
    }

    @Override
    public int size() {
        return size;
    }

    @Override
    public void add(T item) {
        if (size == array.length) {
            resize();
        }
        array[size] = item;
        size++;
    }

    @Override
    public void add(T item, int index) {

        if (size == array.length || index > size) {
            Object[] tempArray = new Object[array.length + vector];
            System.arraycopy(array, 0, tempArray, 0, index);
            tempArray[index] = item;
            System.arraycopy(array, index, tempArray, index + 1, size - index);
            array = tempArray;
        } else {
            System.arraycopy(array, 0, array, 0, index);
            System.arraycopy(array, index, array, index + 1, size - index);
            array[index] = item;
        }
        size++;
    }

    @Override
    @SuppressWarnings("unchecked")
    public T remove(int index) {
        T temp = (T) array[index];
        if (index == array.length - 1) {
            array[index] = null;
        } else {
            System.arraycopy(array, index + 1, array, index, size - 1);
        }
        size--;
        return temp;
    }

    @Override
    @SuppressWarnings("unchecked")
    public T get(int index) {
        return (T) array[index];
    }

    private void resize() {
        Object[] newArray = new Object[array.length + vector];
        System.arraycopy(array, 0, newArray, 0, array.length);
        array = newArray;
    }

}
