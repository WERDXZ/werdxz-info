// Main entry point - loads resume data and registers custom elements

// Fetch resume data
let resumeData = null;

async function loadResumeData() {
    try {
        // Use API instead of direct cloud bucket access
        // Forward any query parameters from the page URL to the API
        const urlParams = new URLSearchParams(window.location.search);
        const apiUrl = new URL('http://localhost:60232/v1/resume');
        apiUrl.search = urlParams.toString();

        const response = await fetch(apiUrl.toString());
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        resumeData = await response.json();
        console.log('Resume data loaded successfully from API');
        return resumeData;
    } catch (error) {
        console.error('Error loading resume data:', error);
        throw error;
    }
}

// Export resume data getter
export function getResumeData() {
    return resumeData;
}

// Custom element: r-header
class RHeader extends HTMLElement {
    async connectedCallback() {
        const data = getResumeData();
        if (!data) return;

        const template = document.getElementById('header-template');
        const content = template.content.cloneNode(true);

        const personal = data.personal;

        content.querySelector('.name').textContent = `${personal.firstName} ${personal.lastName}`;
        content.querySelector('.email').textContent = personal.email;
        content.querySelector('.phone').textContent = personal.phone;
        content.querySelector('.location').textContent = personal.location;

        const website = content.querySelector('.website');
        website.href = personal.website;
        website.textContent = 'Website';

        const github = content.querySelector('.github');
        github.href = `https://github.com/${personal.github}`;
        github.textContent = 'GitHub';

        const linkedin = content.querySelector('.linkedin');
        linkedin.href = `https://linkedin.com/in/${personal.linkedin}`;
        linkedin.textContent = 'LinkedIn';

        this.appendChild(content);
    }
}

// Custom element: r-education
class REducation extends HTMLElement {
    async connectedCallback() {
        const data = getResumeData();
        if (!data) return;

        const title = document.createElement('h2');
        title.id = 'education-heading';
        title.className = 'section-title';
        title.textContent = 'Education';
        this.appendChild(title);

        const template = document.getElementById('education-entry-template');

        data.education.forEach(edu => {
            const content = template.content.cloneNode(true);

            content.querySelector('.institution').textContent = edu.institution;

            const startDate = formatDate(edu.startDate);
            const endDate = edu.endDate ? formatDate(edu.endDate) : 'Present';
            content.querySelector('.dates').textContent = `${startDate} - ${endDate}`;

            content.querySelector('.degree').textContent = edu.degree;

            if (edu.minors && edu.minors.length > 0) {
                const minorsText = edu.minors.map(m => `Minor in ${m}`).join(' & ');
                content.querySelector('.minors').textContent = ` & ${minorsText}`;
            }

            content.querySelector('.location').textContent = edu.location;

            if (edu.gpa) {
                content.querySelector('.gpa').textContent = `GPA: ${edu.gpa}`;
            }

            this.appendChild(content);
        });
    }
}

// Custom element: r-experience
class RExperience extends HTMLElement {
    async connectedCallback() {
        const data = getResumeData();
        if (!data) return;

        const title = document.createElement('h2');
        title.id = 'experience-heading';
        title.className = 'section-title';
        title.textContent = 'Experience';
        this.appendChild(title);

        const template = document.getElementById('experience-entry-template');

        data.experience.forEach(exp => {
            const content = template.content.cloneNode(true);

            content.querySelector('.title').textContent = exp.title;

            const startDate = formatDate(exp.startDate);
            const endDate = exp.endDate ? formatDate(exp.endDate) : 'Now';
            content.querySelector('.dates').textContent = `${startDate} - ${endDate}`;

            content.querySelector('.organization').textContent = exp.organization;

            if (exp.location) {
                content.querySelector('.location').textContent = exp.location;
            }

            if (exp.description) {
                content.querySelector('.description').textContent = exp.description;
            } else {
                content.querySelector('.description').remove();
            }

            const bulletsList = content.querySelector('.bullets');
            if (exp.bullets && exp.bullets.length > 0) {
                exp.bullets.forEach(bullet => {
                    const li = document.createElement('li');
                    li.textContent = bullet;
                    bulletsList.appendChild(li);
                });
            } else {
                bulletsList.remove();
            }

            this.appendChild(content);
        });
    }
}

// Custom element: r-projects
class RProjects extends HTMLElement {
    async connectedCallback() {
        const data = getResumeData();
        if (!data) return;

        const title = document.createElement('h2');
        title.id = 'projects-heading';
        title.className = 'section-title';
        title.textContent = 'Projects';
        this.appendChild(title);

        const template = document.getElementById('project-entry-template');

        data.projects.forEach(project => {
            const content = template.content.cloneNode(true);

            content.querySelector('.title').textContent = project.title;

            // Date and status
            const dateEl = content.querySelector('.date');
            const statusBadge = content.querySelector('.status-badge');

            dateEl.textContent = formatDate(project.date);

            if (project.status === 'active') {
                statusBadge.textContent = 'Active';
            } else if (project.status === 'archived') {
                statusBadge.textContent = 'Archived';
            } else if (project.status === 'completed') {
                statusBadge.textContent = 'Completed';
            } else if (project.status === 'unmaintained') {
                statusBadge.textContent = 'Unmaintained';
            } else {
                statusBadge.remove();
            }

            // GitHub link
            const githubLink = content.querySelector('.github-link');
            if (project.github) {
                githubLink.href = `https://github.com/${project.github}`;
                githubLink.textContent = 'GitHub';
            } else {
                githubLink.remove();
            }

            // Live URL
            const liveLink = content.querySelector('.live-link');
            if (project.liveUrl) {
                liveLink.href = project.liveUrl;
                liveLink.textContent = 'Live Site';
            } else {
                liveLink.remove();
            }

            content.querySelector('.description').textContent = project.description;

            const bulletsList = content.querySelector('.bullets');
            if (project.bullets && project.bullets.length > 0) {
                project.bullets.forEach(bullet => {
                    const li = document.createElement('li');
                    li.textContent = bullet;
                    bulletsList.appendChild(li);
                });
            } else {
                bulletsList.remove();
            }

            this.appendChild(content);
        });
    }
}

// Custom element: r-extracurricular
class RExtracurricular extends HTMLElement {
    async connectedCallback() {
        const data = getResumeData();
        if (!data) return;

        const title = document.createElement('h2');
        title.id = 'extracurricular-heading';
        title.className = 'section-title';
        title.textContent = 'Extracurricular';
        this.appendChild(title);

        const template = document.getElementById('extracurricular-entry-template');

        data.extracurricular.forEach(activity => {
            const content = template.content.cloneNode(true);

            content.querySelector('.title').textContent = activity.title;
            content.querySelector('.dates').textContent = activity.dates;
            content.querySelector('.organization').textContent = activity.organization;

            const websiteLink = content.querySelector('.website-link');
            if (activity.website) {
                websiteLink.href = activity.website;
                websiteLink.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>';
            } else {
                websiteLink.remove();
            }

            content.querySelector('.description').textContent = activity.description;

            const achievementsList = content.querySelector('.achievements');
            if (activity.achievements && activity.achievements.length > 0) {
                activity.achievements.forEach(achievement => {
                    const li = document.createElement('li');
                    li.textContent = achievement;
                    achievementsList.appendChild(li);
                });
            } else {
                achievementsList.remove();
            }

            this.appendChild(content);
        });
    }
}

// Helper function to format dates
function formatDate(dateString) {
    if (!dateString) return '';
    const [year, month] = dateString.split('-');
    const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    return month ? `${monthNames[parseInt(month) - 1]} ${year}` : year;
}

// Print dialog functionality
function setupPrintDialog() {
    const printBtn = document.getElementById('print-btn');
    const printDialog = document.getElementById('print-dialog');
    const printConfirmBtn = document.getElementById('print-confirm-btn');
    const printCancelBtn = document.getElementById('print-cancel-btn');

    printBtn.addEventListener('click', () => {
        printDialog.showModal();
    });

    printConfirmBtn.addEventListener('click', () => {
        printDialog.close();
        window.print();
    });

    printCancelBtn.addEventListener('click', () => {
        printDialog.close();
    });

    // Close on backdrop click
    printDialog.addEventListener('click', (e) => {
        if (e.target === printDialog) {
            printDialog.close();
        }
    });
}

// Load data and register custom elements
loadResumeData().then(() => {
    customElements.define('r-header', RHeader);
    customElements.define('r-education', REducation);
    customElements.define('r-experience', RExperience);
    customElements.define('r-projects', RProjects);
    customElements.define('r-extracurricular', RExtracurricular);
    console.log('All custom elements registered');

    // Setup print dialog after DOM is ready
    setupPrintDialog();
}).catch(error => {
    console.error('Failed to initialize resume app:', error);
    document.body.innerHTML = '<p style="padding: 2rem; color: red;">Error loading resume. Please check console.</p>';
});
