SRC_DIR := src
OUT_DIR := out

SRCS := $(wildcard $(SRC_DIR)/*/*.java)
CLASSES := $(SRCS:$(SRC_DIR)/%.java=$(OUT_DIR)/%.class)

JC := javac
JCFLAGS := -d $(OUT_DIR)/ -cp $(SRC_DIR)/

.SUFFIXES: .java .class

.PHONY: all
all: $(CLASSES)
	echo $(CLASSES)

$(CLASSES): $(OUT_DIR)/%.class: $(SRC_DIR)/%.java $(OUT_DIR)
	$(JC) $(JCFLAGS) $<

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

run/%: $(OUT_DIR)/%.class
	CLASSPATH=$(OUT_DIR) java $*

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)
