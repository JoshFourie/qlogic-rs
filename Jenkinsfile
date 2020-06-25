pipeline {
    agent { docker { image 'rust' } }
    stages {
        stage('build') {
            steps {
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