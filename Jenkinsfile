pipeline {
    agent { docker { image 'rust' } }
    triggers {
        cron('H 2 * * *')
    }
    stages {
        stage('sanity') {
            steps {
                sh 'curl https://github.com/rust-lang/crates.io-index'
            }
        }
        stage('build') {
            steps {
                sh 'cargo check'
                sh 'cargo build --release'
            }
        }
        stage('test') {
            steps {
                sh 'cargo test --all'
            }
        }
    }  
}
