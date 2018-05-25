n = int(input("Inserisci un numero per sapere se ci piace: "))
if n <= 9:
    print("Non ci piace :c")
else:
    cifre = [int(x) for x in str(n)]

    def pairwise(lista):
        """
        Questa funzione crea una lista di tuple contenenti elementi
        a due a due adiacenti tra loro in una lista di partenza.

        Questa funzione esiste anche nel modulo itertools
        """
        out = []
        for i in range(len(lista)-1):
            out.append((lista[i], lista[i+1]))
        return out

    data = map(lambda x: abs(x[0]-x[1]) == 1, pairwise(cifre))
    if all(data):
        print("Ci piace!")
    else:
        print("Non ci piace :c")