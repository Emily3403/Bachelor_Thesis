# BA Motivation
> Warum mache ich das eigentlich?
- Performance
    - Latenz: Schneller den interrupt handlen können
- FIQ Nutzbar machen



## Warum Userspace?
- Warum keinen klassischen Treiber im Kernel Space? ✓
- Irgendwie ist das einer der intrinsischen  Motivationen, aber ich kann mir gar nicht wirklich erklären was daran so gut ist. Klar: Kernel Code ist anfällig aber warum mache ich nicht den Interrupt Vergleich im Kernel Space? Sollte das nicht deutlich besser funktionieren? ✓
> **A:** Das UIO Framework mit diesem Patch zu versehen ist tatsächlich sehr sinnvoll, da es die sinnvollste Art ist Userspace Treiber mit memory mapping zu implementieren und dort performance zu erhöhen ein krasser bonus wäre

- Warum ist es schneller?
> **A:** (→ = copy)
> Normaler Stack: Device → Kernel → User Application
> UIO Stack: Device → User
>
> Dadurch, dass der Speicher direkt im Userspace ankommt und mit `/dev/uioX` `mmap`-able ist, können alle Userspace Anwendungen sich den gleichen (physischen) Speicher teilen

- Warum UIO? Gibt es noch andere alternativen?
> **A:** UIO gibt ein sehr schönes Interface: `mmap()` den Speicher des Geräts und interrupt über `read()`. 
> 
> Alternativen:
> - Register von UART (`mmap`): Direkt auf `/dev/mem` zugreifen
> - Udev ist nur für hotplug
 
# Hauptmotivation
- Ist Userspace oder FIQ die Hauptmotivation?
- Latenz, 
- Syscall nice real-time priority


# Performance
- Ist das wirklich das Alleinstellungsmerkmal? → Ja
- Will ich mich so sehr auf Kontext Wechsel versteifen? Ist nicht FIQ einfach grundsätzlich schneller und konsistenter, da er eine hörere Prio hat? → Ja
    - Schnell in den Treiber-Code springen mit PC anspringen, priviledges droppen, handlen und danach mit udf#42 zurück



# Ziele
- Ich würde gerne, dass mein FIQ Userspace Treiber ähnlich schnell ist wie der vergleichbare Kernel Treiber
- Ich würde gerne die Unterschiede in Performance von Raspi 3b+ zu 4







# Konkrete Fragen
- Soll ich eigentlich die Buch LaTeX Vorlage nutzen um `\chapter` zu machen? → Nein Gibts von sect ne Vorlage? → Ja
- Wie nenne ich meine Arbeit im Text? → thesis (klein)
- Brauch ich Motivation warum ich einen Treiber im Userspace schreiben will? So Advantage / Disadvantage mäßig? → Trusted Computing Base (TCB), Kein Vergleich von Rust im Kernel
- Was wäre der Vorteil von Treiber für Netzwerkkarte?
    - Raspi hat 100mbit ethernet onboard, kann man damit überhaupt sinnvoll Dinge messen?
- Welche anderen Treiber könnte ich noch implementieren?
    - Touchscreen? → Latenz
- Wie baue ich UIO vs FIQ auf? Welches kommt als erstes? → UIO (polling), UIO (IRQ), UIO (FIQ)
- Wie detailliert soll ich den "Hello World" UIO Treiber aufschlüsseln? Braucht es so viel konkreten Code? → Abstraktionsniveau über Technischer Dokumentation, Inhalt der Arbeit sollte nicht ausreichen um meinen Code konkret nachzubauen, es soll versucht werden textuell zu beschreiben.
- Wie soll ich Implementation strukturieren? In kleinen Schritten nach und nach die Treiber erarbeiten? Oder mehr so als overview und dann in kleine Bereiche mal reindippen? → Die Chronologie darf in der Arbeit nicht erkennbar sein





# Warum Userspcae
- Ursprüngliche motivation: Securtiy. Um so mehr priviligierten Code, desto größer die Angriffsfläche
- Gerätetreiberber im Userspace ist Security 
- Die Verbindung zwischen FIQ
- Latenz, Kontext Wechsel zieht Zeit
- Damals: Interrupt geht nicht schnell durch
- Handler Code kann extrem schlank sein für FIQ, wird schneller: Latenz minimieren
- Paul Emmerich Paper nochmal anschauen
