# Notes
# Draft Part A
1. Parsen der Karten in ein Format das ähnlich der Gamedata aus Aufgabe 3 ist
   1. Split bei ": " 
      1. Linker Part row_key
      2. rechter key Spiele
   2. Split bei " | "
      1. links : `winning numbers`
      2. rechts: `numbers i have`
   3. Split beide Seiten bei " " und einzelne Zahlen herausfinden (nicht möglich da einzelne nr da sind, regex `\d` )
2. Schreiben einer Funktion die aus einem 2 Arrays aus i32 die matches sucht und `2^len(matches) returned außer es ist länge 0, dann 0 returnen`
3. Für jeden Eintrag in der Eingabe:
   4. Ermittle die Gewinnsumme (Funktion aus Zeile 2)
   5. Speichere Gewinn des Spiels(Zeile) in einem vector
6. Bilde Summe alle EInträge in diesem Vector

# Draft Part B

1. Für jede Karte im Original
   1. Berechne Matches und adde matches
   2. für jedes Match => Berechne Matches
2. Summe über alle matches