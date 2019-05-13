public class MatrixArray<T> implements IArrays<T> {

    private int size;
    private int lineLength;
    private IArrays<IArrays<T>> arrays;

    public MatrixArray(int lineLenght) {
        this.lineLength = lineLenght;
        arrays = new SingleArray<>();
        size = 0;
    }

    public MatrixArray() {
        this(10);
    }

    @Override
    public int size() {
        return size;
    }

    @Override
    public void add(T item) {
        if (size == arrays.size() * lineLength) {
            resize();
        }

        arrays.get(size / lineLength).add(item);
        size++;
    }

    @Override
    public void add(T item, int index) {
    // TODO сделать перенос по матрице, если на линии мест больше нет.
        if (size == arrays.size() * lineLength) {
            resize();
        }
        arrays.get(index / lineLength).add(item, index % lineLength);
        size++;
    }

    @Override
    public T remove(int index) {
        T temp = arrays.get(index / lineLength).remove(index % lineLength);
        size--;
        return temp;
    }

    @Override
    public T get(int index) {
        return arrays.get(index / lineLength).get(index % lineLength);
    }

    private void resize() {
        arrays.add(new VectorArray<T>(lineLength));
    }
}
