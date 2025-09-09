def predict(x):
    return [i ** 2 for i in x]

if __name__ == "__main__":
    input_data = [1, 2, 3, 4]
    result = predict(input_data)
    print("Predicted:", result)
