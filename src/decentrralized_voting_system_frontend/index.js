
        async function addUser() {
            const userName = document.getElementById('userName').value;
            const response = await fetch('/path/to/add_user', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ userName }),
            });

            const result = await response.json();
            console.log(result);
        }

        async function addBallot() {
            const electionId = document.getElementById('electionId').value;
            const options = document.getElementById('options').value.split(',');
            const startTime = document.getElementById('startTime').value;
            const endTime = document.getElementById('endTime').value;

            const response = await fetch('/path/to/add_ballot', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ electionId, options, startTime, endTime }),
            });

            const result = await response.json();
            console.log(result);
        }

        async function castVote() {
            const voterId = document.getElementById('voterId').value;
            const candidate = document.getElementById('candidate').value;
            const voteElectionId = document.getElementById('voteElectionId').value;

            const response = await fetch('/path/to/cast_vote', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ voterId, candidate, voteElectionId }),
            });

            const result = await response.json();
            console.log(result);
        }

        async function calculateResults() {
            const resultElectionId = document.getElementById('resultElectionId').value;

            const response = await fetch(`/path/to/calculate_results/${resultElectionId}`);
            const result = await response.json();

            const resultContainer = document.getElementById('resultContainer');
            resultContainer.innerHTML = JSON.stringify(result, null, 2);
        }
