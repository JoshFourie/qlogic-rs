pipeline {
    agent { docker { image 'rust' } }
    triggers {
        cron('25 16 * * *')
    }
    stages {
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