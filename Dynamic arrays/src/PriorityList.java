public class PriorityList<T> {


    private int size;
    private Link[] array;

    public PriorityList() {
        size = 0;
        array = new Link[5];
        for (int i = 0; i < 5; i++) {
            array[i] = new Link<T>();
        }
    }

    public int size() {
        return size;
    }

    @SuppressWarnings("unchecked")
    public void add(int priority, T item) {
        array[priority].add(item);
    }

    @SuppressWarnings("unchecked")
    public T get() {
        T item = null;
        for (Link link : array) {
            item = (T) link.pop();
            if (item != null) {
                break;
            }
        }
        return item;
    }


    private static class Link<T> {
        private Node<T> firstElement;
        private Node<T> lastElement;

        void add(T element) {
            Node<T> tempElement = new Node<>(element);
            if (firstElement == null) {
                firstElement = tempElement;
                lastElement = tempElement;
            } else if (firstElement.nextElement == null) {
                firstElement.nextElement = tempElement;
                lastElement = tempElement;
            } else {
                lastElement.nextElement = tempElement;
                lastElement = lastElement.nextElement;
            }
        }

        T pop() {
            if (firstElement == null) {
                return null;
            }
            T temp = firstElement.element;
            firstElement = firstElement.nextElement;
            return temp;
        }

    }

    private static class Node<T> {
        T element;
        Node<T> nextElement;

        Node(T element) {
            this.element = element;
        }
    }
}
