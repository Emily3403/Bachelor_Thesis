# Was mache ich?

- Ich möchte den FIQ auf ARM einsetzen, um Treiber im User-Space zu beschleunigen
  - Als erstes möchte ich das für die UArt machen. 
  - Wenn dann geht es danach mit IRQ weiter. Vielleicht aber auch nicht


# Motivation das zu machen
- Hauptmotivation: Code verringern der im Kernel läuft (Trusted Computing Base)
  - Paul Emmerich Paper als Referenz?
  - Kernel ist schwerer updatebar als Userspace
  - Kernel Debugging ist *schwer*
- Hinkriegen durch Userspace Treiber
- Sind mitunter sehr langsam
- Speedup durch besseren FIQ / IRQ Stack
- Diese Technik erlaubt es Treiber, die klassischerweise im Kernel wegen low-latency laufen müssten, in den userspace ausgelagert zu werden
  - Caveat: Da nur eine Interrupt Source unterstützt wird, kann auch immer nur genau ein Treiber in den userspace ausgelagert werden
  - Vielleicht gilt das wegen IRQs dann nicht mehr



# Implementation
1. Eigentlich ist die Implementierung ziemlich einfach
   - Es existiert ja bereits ein Framework für userspace treiber, nämlich [UIO](../Documentation/driver-api/uio-howto.rst)
   - UIO Dateien liegen als `/dev/uioX` bereit
   - Interrupt kann bekommen werden indem auf `/dev/uioX` gelesen wird, integer zurück = interrupt count
2. Wir können einen eigenen FIQ Handler schreiben, der in dieses Framework eingreift und schneller die Interrupts zustellt
3. Mit irgendeinem effizienten Algorithmus (reicht der `read()` bereits? Bringt `select()` etwas?) lässt sich bestimmt ein sehr low overhead uart treiber implementieren

# Tatsächliche Implementation
- Um Uart zu sprechen brauche ich ja ein zweiten raspi, der mir Dinge sendet damit ich sie überhaupt empfangen kann, richtig?
  - → Nicht notwendigerweise, USB würde auch gehen 
- Wie kann ich die _grundsätzlich_, also mit zwei stock treibern, miteinander reden lassen? Was für Programme sind da empfehlenswert?
- Für den Benchmark, wie viele Pakete ohne Überlauf empfangen wurden ist, denke ich folgender Algorithmus optimal:
  - C = Client (userspace driver), S = Server (referenz-maschine)
  - Client sendet `I`, das initiierungs-paket. Dieses besitzt einen Seed, mit im random-generator vom server genutzt wird 
  - S sendet `C`onfirmation
  - nach einem zufälligen Interval `[0.25, 1]` fängt der Server an wie wild zufällige Pakete zu senden, so schnell es geht
  - Der Client versucht so gut es geht alle Pakete zu empfangen
  - Nachdem der Client das letzte Paket empfangen hat und die FIFO leer ist, sendet er ein abschließendes Paket, welches wiederum `ACK`ed wird, um die Übertragung zu beenden
  - Abschließend kann der Client abgleichen wie viele Pakete er (nicht) bekommen hat indem er selbst die gesamten Pakete generiert und schaut welche fehlen  
- Metriken
  - RTT
  - Throughput



# Gespräch

- Wenn FIQ 1 zu 1:
- Dann Werte für User thread auf FIQ Stack und mit Load Multiple in einer instruktion ladbar
- Wie wird interrupt gecleared? → Schwierigkeit → udf #42?


- Als Erstes wollen wir die Implementierung mit der normalen Interrupt struktur machen: FIQ hoch, dann runter, dann erst in der userspace. Nicht direkt von FIQ zu user.
- Dafür Scheduler bescheid sagen, dass der Thread jetzt dran ist.

