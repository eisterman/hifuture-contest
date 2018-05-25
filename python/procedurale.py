n = int(input("Inserisci un numero per sapere se ci piace: "))
if n <= 9:
    print("Non ci piace! :c")
else:
    cifre = [int(x) for x in str(n)]

    for i in range(len(cifre)-1):
        if abs(cifre[i]-cifre[i+1]) != 1:
            print("Non ci piace! :c")
            break
    else:
        print("Ci piace!")