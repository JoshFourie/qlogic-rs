pipeline {
    agent { docker { image 'rust' } }
    triggers {
        cron('45 16 * * *')
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