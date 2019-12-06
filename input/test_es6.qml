import QtQuick 2.0

Item {
    id: root

    function foo (bar) {
        const tmp = [1,2,3];
        return [...tmp, bar];
    }

    Rectangle {
        id: rect
        color: {
            switch (arg) {
            case 0: return "red";
            case 1: return "green";
            case 2: return "blue";
            }
        }
        Component.onCompleted: {
            const a = 2;
            const b = 1;
            const res = (a > b ? { a, b } : { });
        }

        readonly property int toto: {
            return (arg => {
                console.log ("OK", arg);
                return (arg ? 1 : 0);
            }) (true);
        }
    }

    function bar (arg) {
        const tmp = {
            foo: "toto",
            bar: "tutu",
        };
        const { foo, bar } = tmp;
        console.log (foo, bar);
    }

    function outter () {
        function inner () {
            console.log ("done");
        }
        inner ();
    }
}
