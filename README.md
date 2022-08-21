# AR-MINTIN-CLI

An interactive command-line application designed for fast long-term memorisation of concepts.

## DESCRIPTION

The application aims to help memorising. Its primary purpose is to reduce the probability of *forgetting* any fact from the given topic. Other important factors are to a maintain a smooth learning curve, maximise the number of ways representing the given fact and do not drop the already learnt facts.

The application would fit into *flashcards* category, although it is not intended to be a classical flashcard software and may deviate at some point of time.

The input data (a topic consisting of facts) is a list of key-value pairs.

The application allows to save the progress into the file to resume later. Edits made to the list file.

The application could also be used as a web client to a remote AR-MINTIN web server, which contains the progress data. 

## LIMITATIONS

At the time of writing, there is no in-built interface to edit the JSON-formatted input file. The syntax can be seen in `demo.json` (see: EXAMPLES).

Customisation of the learning process (outside key-value pairs) is currently under development.

## EXAMPLES

The following is the example of the data file.

`demo.json`
```json
{
    "version": 1,
    "data": [ 
    [ "alfa1", "beta1" ],
    [ "alfa2", "beta2" ],
    [ "alfa3", "beta3" ],
    [ "alfa4", "beta4" ],
    [ "alfa5", "beta5" ]
    ]
}
```

### Standard way of running the application and saving progress

```bash
ar-mintin demo.json -p progress.p
```

### Running the application, reading the progress and saving elsewhere
```bash
ar-mintin demo.json -p progress.p -o out_progress.p
```

### Running the application as a web client to the remote server
```bash
ar-mintin https://www.example.com:8083
```

## NOTES

Feel free to contribute, fork or suggest new features.

*Made with <3 by Arnoldas Rauba, 2022*
