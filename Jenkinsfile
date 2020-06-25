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
                sh 'cargo test'
            }
        }
        post {
            always {
                echo 'The Enraged Magic Carp has...'
            }
            success {
                echo 'Succeeded!'
            }
            failure {
                echo 'Failed.'
            }
        }
    }
}