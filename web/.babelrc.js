module.exports = {
    "presets": [
        [
            "@babel/preset-env",
            {
                "targets": {
                    "edge": "17",
                    "firefox": "60",
                    "chrome": "67",
                    "safari": "11.1"
                },
                "useBuiltIns": "entry",
                "corejs": "3" // 声明corejs版本
            }
        ],
        [
            "@babel/preset-react",
            {
                "runtime": "automatic"
            }
        ],
        [
            "@babel/preset-typescript"
        ]
    ],
    "plugins": [
        [
            "@babel/plugin-proposal-decorators",
            {
                "legacy": true
            }
        ],
        [
            "import",
            {
                "libraryName": "antd",
                "libraryDirectory": "es",
                "style": true
            },
            "antd"
        ],
        [
            "import",
            {
                "libraryName": "@antv/x6-react-components",
                "libraryDirectory": "lib", // es or lib
                "style": true,
                "transformToDefaultImport": false
            },
            "antv"
        ]
    ].concat(process.env.NODE_ENV === 'dev' ? [] : [
        "transform-remove-console"
    ])
}