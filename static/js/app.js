// Variáveis globais
let currentUser = null;
let currentNFT = null;
let selectedBlockchain = null;

// Elementos do DOM
const loginBtn = document.getElementById('login-btn');
const logoutBtn = document.getElementById('logout-btn');
const blockchainSelect = document.getElementById('blockchain-select');
const nftForm = document.getElementById('nft-form');
const nftDisplay = document.getElementById('nft-display');
const nftContent = document.getElementById('nft-content');
const mintBtn = document.getElementById('mint-btn');

// Função para carregar as blockchains disponíveis
async function loadBlockchains() {
    try {
        const response = await fetch('/blockchains');
        const blockchains = await response.json();
        
        // Limpar opções existentes
        blockchainSelect.innerHTML = '';
        
        // Adicionar opções para cada blockchain
        blockchains.forEach(blockchain => {
            const option = document.createElement('option');
            option.value = blockchain.name.toLowerCase();
            option.textContent = blockchain.name;
            blockchainSelect.appendChild(option);
        });
        
        // Selecionar a primeira blockchain por padrão
        if (blockchains.length > 0) {
            selectedBlockchain = blockchains[0].name.toLowerCase();
            blockchainSelect.value = selectedBlockchain;
        }
    } catch (error) {
        console.error('Erro ao carregar blockchains:', error);
        blockchainSelect.innerHTML = '<option value="">Erro ao carregar blockchains</option>';
    }
}

// Função para realizar login com PassKey
async function loginWithPassKey() {
    try {
        // Aqui você implementaria a lógica de login com PassKey
        // Por enquanto, vamos simular um login bem-sucedido
        currentUser = {
            id: 'user-123',
            name: 'Usuário Teste'
        };
        
        // Atualizar UI
        loginBtn.style.display = 'none';
        logoutBtn.style.display = 'inline-block';
        
        alert('Login realizado com sucesso!');
    } catch (error) {
        console.error('Erro no login:', error);
        alert('Erro ao realizar login');
    }
}

// Função para realizar logout
function logout() {
    currentUser = null;
    currentNFT = null;
    
    // Atualizar UI
    loginBtn.style.display = 'inline-block';
    logoutBtn.style.display = 'none';
    nftDisplay.style.display = 'none';
    
    alert('Logout realizado com sucesso!');
}

// Função para criar NFT
async function createNFT(event) {
    event.preventDefault();
    
    if (!currentUser) {
        alert('Por favor, faça login primeiro');
        return;
    }
    
    if (!selectedBlockchain) {
        alert('Por favor, selecione uma blockchain');
        return;
    }
    
    // Obter dados do formulário
    const name = document.getElementById('nft-name').value;
    const description = document.getElementById('nft-description').value;
    const image = document.getElementById('nft-image').value;
    
    // Criar objeto de metadados
    const metadata = {
        name: name,
        description: description,
        image: image,
        attributes: []
    };
    
    try {
        // Enviar requisição para criar NFT
        const response = await fetch('/nfts', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(metadata)
        });
        
        if (!response.ok) {
            throw new Error('Erro ao criar NFT');
        }
        
        const nft = await response.json();
        currentNFT = nft;
        
        // Exibir NFT criado
        displayNFT(nft);
    } catch (error) {
        console.error('Erro ao criar NFT:', error);
        alert('Erro ao criar NFT');
    }
}

// Função para exibir NFT
function displayNFT(nft) {
    nftContent.innerHTML = `
        <h3>${nft.metadata.name}</h3>
        <p>${nft.metadata.description}</p>
        <img src="${nft.metadata.image}" alt="${nft.metadata.name}" style="max-width: 300px;">
        <p>Blockchain: ${nft.blockchain}</p>
        <p>ID: ${nft.id}</p>
    `;
    
    nftDisplay.style.display = 'block';
}

// Função para mintar NFT
async function mintNFT() {
    if (!currentUser || !currentNFT) {
        alert('Nenhum NFT criado');
        return;
    }
    
    try {
        // Aqui você implementaria a lógica para mintar o NFT
        // Por enquanto, vamos simular o processo
        alert(`NFT mintado na blockchain ${selectedBlockchain}!`);
    } catch (error) {
        console.error('Erro ao mintar NFT:', error);
        alert('Erro ao mintar NFT');
    }
}

// Event listeners
loginBtn.addEventListener('click', loginWithPassKey);
logoutBtn.addEventListener('click', logout);
blockchainSelect.addEventListener('change', (event) => {
    selectedBlockchain = event.target.value;
});
nftForm.addEventListener('submit', createNFT);
mintBtn.addEventListener('click', mintNFT);

// Carregar blockchains ao iniciar
loadBlockchains();