Ja dies das meine "Bachelor Arbeit" auf ARM nen FIQ und UIO irgendwie nutzbar machen. 

Ist ziemlich kompliziert die Mini-UART in betrieb zu nehmen und einen kompletten Treiber dafür mit tock_registers in Rust zu programmieren. Naja, hab ich halt gemacht. Ich bin Emily Aurelia Seebeck mit der Matrikelnummer 411364.

Die vergesse ich auch nicht. Keine Ahnung warum, aber die Zahl kann ich mir richtig gut merken. Sollte mal ein Schloss-Code für was werden. Naja, ich schweife ab. Wo waren wir? Richtig: Bachelorarbeit.
Was will ich hier noch machen?

Ich glaub der FIQ wär interessant. Weil das Assembly-Trampolin ist halt insofern ungenutzt, als dass FIQ direkt zu IRQ umgeroutet wird und die FIQ beschleunigung gar nicht zunutze gemacht werden kann.
Wegen JJ von NVIDIA möchte ich unter ARM den FIQ in UIO implementieren, damit ein Rack{3} with ARM laufen kann. GAaaaanz viele Raspis in meinem Büro, Serverraum und Firewall. Dazu noch so'n paar 1024-Core Machines wie Don und Mos [wir brauchen neuen ECC DDR4 RAM] und wir sind glücklich

